# M1 — Open questions

Milestone-scoped. Cross-milestone questions live in
[`../open_questions.md`](../open_questions.md).

## Index

| Q   | Title                                                                                | Resolved in        |
|-----|--------------------------------------------------------------------------------------|--------------------|
| Q4  | Rule presentation language — Python functions, graph-rewrite DSL, or Horn clauses?    | P1.3 S1.3.1        |
| Q5  | What "enough" means for the rule set                                                  | P1.3 S1.3.2 + P1.7 |
| Q6  | Symmetry breaking — before or after trace generation?                                 | P1.5 S1.5.3        |
| Q15 | Forward-chaining queue vs priority by cheapness vs random rule ordering               | P1.3 S1.3.3        |
| Q16 | Are rules universal, or per-puzzle?                                                   | P1.3 + P1.7        |
| Q17 | Spatial-relation formalisation — Allen, RCC, or ad-hoc 1-D interval?                  | P1.4 S1.4.2        |
| Q18 | Provenance granularity — per-edge, per-step, or per-derivation-DAG?                   | P1.2 S1.2.3        |
| Q19 | Hypothesis branching — eager (every choice) vs lazy (only when saturation stalls)?    | P1.5 S1.5.1        |
| Q20 | Trace reordering — engine order, planner-pass reorder, or human-template fitting?     | P1.6 S1.6.4        |
| Q21 | IR ↔ DOT structural isomorphism — bidirectional, layout-free                          | S1.1.1 T1.1.1.6    |

---

## Q4 — Rule presentation language

Per [idea 06 §Open sub-questions point 1](../../docs/ideas/06-inference-rules-completeness.md).

**Options:**

- **A** — Python predicate over a `Graph` view. Easy; awful for
  trace generation (the rule "fires" with no structured premise list).
- **B** — Graph-rewrite DSL (LHS pattern → RHS pattern, à la DPO).
  Structural, formal, slightly heavy. Matches
  [idea 07 Reading C](../../docs/ideas/07-categorical-formulation.md).
- **C** — Horn-clause / Datalog encoding. Familiar declarative;
  loses the "graph rewrite" intuition.

**Working answer**: B — a small DSL where each rule is
`(rule <name> :match <pattern> :assert <conclusion> :why <reason-template>)`
with a Python-callable fallback for the cases where a pattern can't
express what we need (initially: the spatial constraint of P1.4).
Locked in P1.3 S1.3.1.

## Q5 — What "enough" means

Per [idea 06 §What "enough" should mean](../../docs/ideas/06-inference-rules-completeness.md):
four non-equivalent answers — functional, propagation, explanation,
domain.

**Working answer**: M1's target is **explanation-complete on Zebra**
(every step of the human walkthrough in
[idea 08](../../docs/ideas/08-human-style-deductive-trace.md) maps to
a named rule firing). Propagation-completeness is desirable but not
required to ship. Domain-completeness is out of scope; revisit per
problem class.

## Q6 — Symmetry breaking

Per [idea 08 §Open questions point 3](../../docs/ideas/08-human-style-deductive-trace.md).
Two paths: (a) pre-trace — engine enforces a canonicalisation
(e.g. house numbering); (b) post-trace — engine explores both,
trace planner annotates "doesn't matter which way numbered".

**Working answer**: ship (a) for M1 with an *escape hatch* — the
engine records a `(symmetry-class …)` derivation note so the trace
planner can label it. (b) is reachable later without rework.
Decided in P1.5 S1.5.3.

## Q15 — Rule ordering

Per [idea 06 §Open sub-questions point 3](../../docs/ideas/06-inference-rules-completeness.md).

**Working answer**: forward-chaining queue with a static priority
class per rule (cheap propagation rules fire before expensive
search-like ones); revisit only if the trace shows obviously bad
ordering. Decided in P1.3 S1.3.3.

## Q16 — Universal vs per-puzzle rules

Per [idea 06 §Open sub-questions point 4](../../docs/ideas/06-inference-rules-completeness.md).

**Working answer**: ship with a *universal core* (composition,
equality, exclusivity, exhaustion, hypothesis-contradiction,
arc-consistency, cardinality, forced-unique) + a *spatial bundle*
loaded when the IR declares a `spatial` ontology. Zebra triggers
both. New problem classes can ship their own bundles via P1.3's DSL.
Decided in P1.3 + P1.7.

## Q17 — Spatial-relation formalisation

The PoC's 2021 open question
([idea 05 §Open question recorded in the README](../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#open-question-recorded-in-the-readme)).

**Options:**

- Allen interval algebra (13 named relations between intervals).
- RCC (region connection calculus).
- Ad-hoc 1-D position lattice with `pos(x) ∈ {1..N}` and integer
  arithmetic on `pos`.

**Working answer**: ad-hoc 1-D position lattice for M1 — it covers
Zebra's `next_to`, `right_of`, `immediately_right_of` cleanly and
maps trivially to the SMT integer encoding in M3. Allen is overkill
for one dimension. Decided in P1.4 S1.4.2.

## Q18 — Provenance granularity

Per [idea 03 §What "contradictions" specifically means](../../docs/ideas/03-three-task-classes.md)
and [idea 08](../../docs/ideas/08-human-style-deductive-trace.md).

**Working answer**: per-derived-edge provenance — each edge carries
a tuple `(rule, premise_edges, source_or_rule_id)`. The full
derivation DAG is recoverable by transitive closure. Decided in P1.2 S1.2.3.

## Q19 — Hypothesis branching strategy

PoC's algorithm branches whenever `solved(s)` returns false. Modern
CDCL solvers branch only when propagation saturates.

**Working answer**: lazy branching — saturate first with all
propagation rules; branch only when no rule fires and the graph is
still incomplete. Mirrors human walkthroughs. Decided in P1.5 S1.5.1.

## Q20 — Trace reordering

Per [idea 08 §Why this is a hard problem — Ordering](../../docs/ideas/08-human-style-deductive-trace.md).

**Working answer**: emit engine-order trace + a `--reorder` flag
that runs a planner pass clustering by entity. Defer full
human-template fitting to M2 (where an LLM surface generator is
available). Decided in P1.6 S1.6.4.

## Q21 — IR ↔ DOT structural isomorphism

Surfaced in S1.1.1 design (2026-05-18). The EBNF ↔ railroad-diagram
move: two surface representations of one structure. The IR is the
textual view; a documented subset of DOT is the graphical view.
Meaning lives in their shared structure, not in either surface.

**Options:**

- **A** — One-way: `ir → dot` only, DOT is for human inspection.
- **B** — One-way with a documented schema: `ir → dot` is total and
  deterministic in structure, `dot → ir` left aspirational.
- **C** — Bidirectional: both `ir → dot` and `dot → ir` are
  deliverables; the project's DOT *subset* and the IR are
  inter-convertible up to layout.

**Working answer**: C. `ir → dot` is mandatory and blocks S1.1.2;
`dot → ir` is required but does not block P1.1 — it lands in P1.2
alongside the typed-hypergraph data model. Only graph *structure* is
fixed by the schema; layout (positions, rank, unspecified style
choices) is free — `random_layout` is permitted.

Pulls forward:

- Each kernel form has a fixed DOT shape — §Rendering in `docs/ir.md`
  (S1.1.1 T1.1.1.6).
- Hyperedges render Levi/bipartite — anchors
  [Q1](../open_questions.md#q1)'s typed-hypergraph + equality-class-ID
  answer visually.
- Rule rendering: 3 modes (side-by-side / DPO span / overlay),
  configurable; `rules.ein` defaults to side-by-side, traces default
  to overlay.
- Trace rendering: 3 views (per-step / aggregate / derivation-DAG),
  configurable; default per-step (matches M1 acceptance §2).
- Branch rendering: tree-of-states for the search-tree view,
  sub-clusters per branch for per-state snapshots.

Decided in S1.1.1 T1.1.1.6.
