# M1 — Core graph reasoning module

**Estimate:** ~3 months (~12 weeks).
**Status:** active — MVP.
**Depends on:** the refactored package skeleton already in `src/ein_bot/`
(commit `d37d039`); nothing else.
**Blocks:** [M2](../m2_nl_to_ir/README.md) (NL frontend needs an IR
target); [M3](../m3_smt_integration/README.md) (SMT slice needs IR +
the "hard-slice" annotation).

## Goal

Re-build the 2021 PoC ([`docs/PoC/README.md`](../../docs/PoC/README.md))
as a *properly engineered, traceable, graph-native* reasoner. The
acceptance criterion is the one set by
[`docs/ideas/08-human-style-deductive-trace.md`](../../docs/ideas/08-human-style-deductive-trace.md):
the engine should not merely return the answer to the Zebra puzzle —
it should reproduce a deductive trace, with named reasoning moves at
every step, of the kind a human would write. The concrete target is
[`examples/README.md`](../../examples/README.md) — the Wikipedia
solution already paired step-by-step with the firing ein rule, branch
depth, and no-good clauses; M1 ships when the engine's emitted trace
covers every row of that table.

Concretely, M1 ships:

1. A small S-expression **IR** for problems, rules, constraints,
   queries, and traces (P1.1).
2. A **typed-hypergraph** core (`ontology` + `fact` +
   `reasoning` layers, provenance-bearing) — per
   [idea 02](../../docs/ideas/02-graph-as-formal-substrate.md) and
   [idea 05](../../docs/ideas/05-zebra-puzzle-graph-reasoner.md) (P1.2).
3. A **rule registry** with the ten rule families catalogued in
   [idea 06](../../docs/ideas/06-inference-rules-completeness.md) (P1.3).
4. A **contradiction detector** that scans the KB for `(X, (not X))`
   pairs and feeds the hypothesis loop (P1.4 — shrunk from the
   original "structural + spatial constraints" scope; the
   structural cases collapsed into `type-exclusivity` shipped by
   P1.3, and the spatial PoC open question resolved declaratively
   via `right-of` / `next-to` + `square-fwd` / `square-bwd` /
   `square-unique` rules per Q17).
5. The **saturate-then-hypothesise loop**, multilevel branching,
   contradiction-with-backjump (P1.5).
6. **Rendering**: DOT for rules / constraints / state /
   state-transitions / search-tree, plus a markdown trace that
   threads them (P1.6).
7. **The Zebra puzzle, end-to-end**, with a generated trace
   matched against the human walkthrough in
   [idea 08 §Target trace](../../docs/ideas/08-human-style-deductive-trace.md) (P1.7).

## Phases

| ID    | Title                                  | Duration | Folder |
|-------|----------------------------------------|----------|--------|
| P1.1  | IR language                            | 1-2 wk   | [`p1.1_ir_language/`](p1.1_ir_language/) |
| P1.2  | Typed-hypergraph core                  | 2 wk     | [`p1.2_typed_hypergraph/`](p1.2_typed_hypergraph/) |
| P1.3  | Inference-rule registry + 10 rules     | 2-3 wk   | [`p1.3_inference_rules/`](p1.3_inference_rules/) |
| P1.4  | Contradiction detection                | 1-2 days | [`p1.4_constraints/`](p1.4_constraints/) (shrunk from ~1 wk per [S1.4.0 review](p1.4_constraints/s1.4.0_review.md)) |
| P1.5  | Hypothesis loop + ATMS branching       | 2 wk     | [`p1.5_hypothesis_loop/`](p1.5_hypothesis_loop/) |
| P1.5a | Zebra solution + saturator/NAF gaps    | unknown  | [`p1.5a_zebra_solution/`](p1.5a_zebra_solution/) (spun out 2026-05-24 from S1.5.8c.7) |
| P1.5b | Set-indexed search engines (monotonic + lattice DAG)               | shipped 2026-05-29 | [`p1.5b_lattice_search/`](p1.5b_lattice_search/) (opened 2026-05-25, closed 2026-05-29 with S1.5b.30 perf round; three public entries `monotonic_solve` / `gaps_solve` / `contradictions_solve` on the unified `inference/monotonic/` engine; tree-solver removal is the next task) |
| P1.6  | Rendering + markdown trace             | 1-2 wk   | [`p1.6_rendering_and_trace/`](p1.6_rendering_and_trace/) |
| P1.7  | Bootstrapping — Zebra end-to-end       | 1-2 wk   | [`p1.7_bootstrapping_zebra/`](p1.7_bootstrapping_zebra/) |
| P1.8  | Improvements (modules+imports+stdlib / COW fork / negative-fact volume; placeholder) | TBD | [`p1.8_ein_lang_modules/`](p1.8_ein_lang_modules/) (directory name historical; phase title broadened 2026-05-21, again 2026-05-22) |
| P1.9  | Hypothesis-loop follow-ups (E1-E23 catalog; placeholder) | TBD | [`p1.9_hypothesis_loop_followups/`](p1.9_hypothesis_loop_followups/) |
| P1.10 | Kernel documentation (IR 4-level split / user-vs-dev / architecture / `docs/index` → `docs/lib` rename / ein-model atoms-vs-objects refinement; placeholder) | TBD | [`p1.10_kernel_docs/`](p1.10_kernel_docs/) (created 2026-05-24 from TODO.md) |
| P1.11 | Package + CLI restructure (`ein-bot`/`ein_bot` → `ein`, merge `ein.py/demo/` into the package, split `cli.py` into a folder; placeholder) | TBD | [`p1.11_package_restructure/`](p1.11_package_restructure/) (created 2026-05-24 from TODO.md) |

Phases run roughly sequentially. P1.6 can start as soon as P1.2 is
in (the renderer only needs the data model); P1.7 is the integration
phase and gates "M1 done". **P1.8 – P1.11 are placeholders.** P1.8
parks improvement themes — modules+imports+stdlib (the original
Q30 deferral from the 2026-05-20 P1.3 review, broadened 2026-05-22
to own the standard library: closure auto-inference deferred whole
from S1.5.5, plus `converse`, the `imply` family, general
totality, reflective rule-implication, type/domain matching),
copy-on-write hypothesis-branch forks, and negative-fact volume
reduction (both surfaced 2026-05-21 during P1.3 / P1.4 work). P1.9
parks the E1-E23
hypothesis-loop catalog spun out of S1.5.4 on 2026-05-21 (closure
refinements, CDCL-inspired learning, search heuristics, CSP-style
pre-processing, engineering/UX), plus the 2026-05-24 mode-taxonomy
+ state-hash-with-hyps additions (E21-E23). P1.10 parks the
kernel-doc reorg surfaced 2026-05-24 (IR 4-level split,
user-vs-dev separation, architecture diagrams, `docs/index/` →
`docs/lib/` rename, ein-model atoms-vs-objects refinement).
P1.11 parks the package + CLI rename also surfaced 2026-05-24
(`ein-bot` → `ein`, demo merge, CLI folder split). None of
P1.8 – P1.11 gates M1 acceptance.

## Acceptance

M1 ships when **all** of the following pass:

1. **IR**: the Zebra puzzle is expressed in a single `.ein` file that
   the parser accepts; round-trip through dump produces byte-identical
   output modulo whitespace and set ordering.
2. **Core**: `ein-bot solve zebra.ein --trace=zebra.md` exits 0,
   writes `zebra.md` + a `zebra/` folder of DOT snapshots, and emits
   the unique solution (Zebra → Japanese; Water → Norwegian).
3. **Trace**: the markdown trace matches the
   [target trace](../../docs/ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
   to within a checklist of *named rule firings* — every move in the
   human walkthrough has a corresponding rule firing in the engine
   trace (matching is structural, not literal).
4. **Three task classes**: `ein-bot query zebra.ein --mode=solve|gaps|contradictions`
   runs all three modes. `gaps` on the puzzle minus condition (15)
   returns at least the colour of house 1; `contradictions` on the
   puzzle plus `(fact (= (color House-1) Green))` returns a minimal
   contradiction with provenance back to that fact and to (5).
5. **Rules**: a `rules.ein` file lists ten named rule families
   ([idea 06](../../docs/ideas/06-inference-rules-completeness.md)
   row-for-row); each is exercised by a Zebra step in the trace
   (P1.7 checks this).
6. **Tests**: pytest suite ≥ 100 tests covering IR / graph / rules /
   constraints / hypothesis / rendering / Zebra; `ruff check .` green.

## Out of scope (deferred)

- NL → IR — [M2](../m2_nl_to_ir/README.md).
- SMT slice — [M3](../m3_smt_integration/README.md).
- Categorical formulation as runtime — [F1](../followups/f1_categorical_formulation.md).
- Self-modifying constraint language — [F2](../followups/f2_self_modifying_language.md).
- Equality saturation / e-graph promotion — beyond P1.2's
  equality-class-id placeholder; revisit if Zebra-class problems
  prove insufficient.

## Cross-cutting decisions made in M1

These get locked in here; later milestones inherit them.

- **IR syntax** (S-expression / homoiconic) — P1.1 S1.1.1.
- **Graph shape** (typed hypergraph with provenance) — P1.2 S1.2.1
  (also resolves [global Q1](../open_questions.md#q1--what-kind-of-graph-is-the-ir)).
- **Rule presentation language** — P1.3 S1.3.1
  (resolves [M1 Q4](open_questions.md)).
- **Trace ordering** (engine order vs reordering pass) — P1.6 S1.6.4.
- **IR encoding** (classic `(type …)` / `(instance …)` *vs* unified
  `is-a`) — **explicitly deferred to P1.7 S1.7.2 T1.7.2.5**.
  Both `examples/zebra.ein` and `examples/zebra2.ein` stay valid
  through every stage of M1; the data model is encoding-agnostic
  (no auto-promotion); the rule registry covers both. The final call
  is made when we have the trace, renderer, and rule-firing
  measurements to compare.

## Design rules-of-thumb (load-bearing across all M1 phases)

These are the user-stated principles that constrain the whole
milestone's design. Stated here so reviewers don't have to chase
them through individual stage files.

| principle                                       | where it surfaces                                                                          |
|-------------------------------------------------|--------------------------------------------------------------------------------------------|
| **Graph is canonical; entity API is derived**   | P1.2 S1.2.1; objects/types/relations all nodes; Levi-bipartite hyperedges; compact view is a render |
| **Graph is static; rules+inference are dynamic**| P1.2 (static KB) vs P1.3+P1.5 (firings + branching) — storage ≠ computation                |
| Types and relations are **first-class** nodes   | P1.2 S1.2.1 (entity model); not derived labels on facts                                    |
| Rules can be **higher-order**                   | params can range over relations (`?rel`); M1 covers first-order + relation-parameter form  |
| Rules are **graph rewrites**                    | P1.3 S1.3.1 (DSL); typed `Pattern` objects with `:match` / `:assert`                       |
| Vars are typed by **premises**, not syntax      | F4 Q35; `(is-a ?var T)` in `:match` rather than `?var:T` sugar                            |
| **Generic > syntactic** when both work          | rule families parametrised over relations, not duplicated per relation (P1.3 S1.3.2)       |
| Syntax should be **as protective as possible**  | P1.1 (grammar rejects malformed IR); P1.2 (loader rejects undeclared types/relations)      |
| Inheritance is **transitive `is-a` propagation**| F4 Q36; collapses "instance-of vs subtype-of" in the unified `is-a` model (zebra2.ein)     |
| Property tags are **rule-application facts**    | `(symmetric R)`, `(square-fwd R)` etc. — not kw-pairs on the relation declaration          |
| **Compound/virtual node kinds — open class**    | M1 ships base only; arch open for sets, projections, groups, top/bottom (M1 Q26)           |
| Composable typed-vars (`?a:T`) — **postponed**  | F4 Q35; pattern language already expresses this                                            |

These principles compose: "types and relations are graph nodes
participating in rules" + "rules are graph rewrites" + "property
tags are facts" produces a self-hosting system where the rule
library is operated on by the same machinery as the puzzle facts.
The unrealised next step is F1 (categorical formulation) which
makes this self-hosting structure formal.

## Risks

- **Trace fidelity drift**: the engine may saturate efficiently but
  in an order no human would write. P1.6 has a *reordering pass* —
  but its acceptance threshold is judgmental. Mitigate by
  hand-picking a small golden trace (5-10 steps) and treating it as
  a regression target.
- **Rule completeness creep**: each family in
  [idea 06](../../docs/ideas/06-inference-rules-completeness.md) is
  small; the integration interactions are not. Budget 30% of P1.3 to
  Zebra-on-a-half-rule-set debugging, not 100% to "implement all ten".
- **Constraint formalisation for spatial**: the [idea 05 open
  question](../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#open-question-recorded-in-the-readme)
  ("Ivory to the left of Green") was unresolved in 2021. **Resolved
  2026-05-18** by [Q17](open_questions.md#q17--spatial-relation-formalisation):
  declarative graph-only via `right-of` / `next-to` + the
  `square-fwd` / `square-bwd` / `square-unique` rules. The
  non-adjacent disjunctive case migrates to P1.5 hypothesis
  branching.

## Open questions

See [`open_questions.md`](open_questions.md) for M1-scoped. Cross-milestone
questions land in [`../open_questions.md`](../open_questions.md).
