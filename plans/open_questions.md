# Open Questions — global (cross-milestone)

Questions that span multiple milestones, are parked indefinitely, or
aren't bound to any specific milestone. Milestone-scoped questions
live in their milestone's `open_questions.md`.

Question IDs are **sticky** — when a question moves to a milestone
file it keeps its id; do not reuse a closed id.

## Index

| Q   | Title                                                                  | Lives in |
|-----|------------------------------------------------------------------------|----------|
| Q1  | What kind of graph is the IR? Typed multigraph, hypergraph, or e-graph? | here (cross-milestone) |
| Q2  | When does the graph engine hand off to a solver, and which one?         | here (cross-milestone; will be operationalised in M3) |
| Q3  | Surface IR syntax — heavy-semantic Lisp vs richer DSL?                  | here (decided in M1.P1.1 S1.1.1) |
| Q4  | Rule presentation language — Python functions, graph-rewrite DSL, or Horn clauses? | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q5  | What "enough" means for the inference rule set                          | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q6  | Should symmetry breaking happen pre- or post-trace?                     | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q7  | Is the surface generator (NL output) allowed to be an LLM?              | [M2](m2_nl_to_ir/open_questions.md) |
| Q8  | Where do ambiguous NL parses go — branched on the IR, or rejected?      | [M2](m2_nl_to_ir/open_questions.md) |
| Q9  | Per-puzzle declared ontology vs ontology inferred from text             | [M2](m2_nl_to_ir/open_questions.md) |
| Q10 | When is direct LLM → constraint emission acceptable (no IR)?            | [M2](m2_nl_to_ir/open_questions.md) |
| Q11 | Does link-grammar enrich LLM input usefully, or is it dead weight?      | [M2](m2_nl_to_ir/open_questions.md) |
| Q12 | Which CT reading does the design commit to (A free / B schema / C functor)? | [followups F1](followups/f1_categorical_formulation.md) |
| Q13 | Self-modifying constraint language — feasibility + bounded problem      | [followups F2](followups/f2_self_modifying_language.md) |
| Q14 | Rule learning source — hand-written, library, LLM-suggested, or learned from walkthroughs? | [followups F4](followups/f4_cross_cutting.md) |
| Q22 | Ontology IR — three sub-heads (relations / definitions / types-and-objects)? | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q23 | What carries an explicit type slot in the data model?                         | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q24 | `:where` clause semantics in `sibling-exclusive`                              | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q25 | Cardinality + ordinality rules with vars — IR shape                          | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q26 | Compound / virtual node kinds for higher-order rules (sets, projections, …)  | [M1](m1_core_graph_reasoning/open_questions.md) |
| Q34 | Algebraic properties beyond symmetric/transitive + 2^7 cartesian product | [followups F4](followups/f4_cross_cutting.md) |
| Q35 | Variable typing via `(is-a ?var Type)` patterns                              | [followups F4](followups/f4_cross_cutting.md) |
| Q36 | Relation inheritance / rule polymorphism                                      | [followups F4](followups/f4_cross_cutting.md) |
| Q37 | Induction — rules from facts                                                  | [followups F4](followups/f4_cross_cutting.md) |
| Q38 | LLM as fact/relation/type/rule extractor                                      | [followups F4](followups/f4_cross_cutting.md) |

---

## Q1 — What kind of graph is the IR?

Per [idea 02 §Open questions](../docs/ideas/02-graph-as-formal-substrate.md)
the candidates are: plain directed graph, typed multigraph, typed
hypergraph (relations span >2 vars), adhesive category, e-graph on
top. The PoC is a typed digraph with set-valued edges; it cannot
natively express `next_to(a, b, c)`-style ternary or `allDifferent`
constraints without faking them.

**Working answer**: **typed hypergraph with provenance and equality
class IDs**. Hyperedges accommodate global constraints; equality-class
IDs leave the door open to e-graph promotion if equality saturation
becomes relevant (per [idea 06 row 2](../docs/ideas/06-inference-rules-completeness.md)).
The decision is finalised in M1.P1.2 S1.2.1.

## Q2 — When does the graph engine hand off?

[Idea 02 §When does the graph stop being enough](../docs/ideas/02-graph-as-formal-substrate.md)
names three threshold conditions: arithmetic-dominated constraints,
large combinatorial search, and global cardinality where a real
solver dominates. The split is *not* "graph-engine for everything
small, solver for everything large" — it's per-sub-problem.

**Working answer**: M1 ships a pure graph-native engine.
M3 introduces SMT-LIB emission for *slices* the graph engine
declares hard (encoded as `(hard-slice ...)` forms in the IR).
A query mode flag selects pure-graph / hybrid / solver-first;
the default in M3 is hybrid.

## Q3 — Surface IR syntax

Decided in M1.P1.1 S1.1.1. The leading candidate is a small
S-expression dialect with named atoms carrying most of the semantic
load (per [idea 01 §Open questions point 3](../docs/ideas/01-self-modifying-constraint-language.md)
recommendation), so a future GBNF grammar (M2.P2.3) is trivial.
Counter-arguments and richer-DSL alternatives are listed there.

---

## How to add a question

1. Pick the smallest scope that contains it. Milestone if it's
   answerable within one milestone, global otherwise.
2. Use the next free `Q<n>` id (continuing the global sequence —
   don't reset per milestone).
3. Add an entry to the index above (or the milestone file's index).
4. Write a short body — what's the question, what answers exist,
   what's the working answer (if any), what would resolve it.
5. Close by editing the body to record the decision and the stage
   that resolved it; keep the id, mark the entry struck-through in
   the index.
