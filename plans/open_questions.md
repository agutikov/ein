# Open Questions — global (cross-milestone)

Questions that span multiple milestones, are parked indefinitely, or
aren't bound to any specific milestone. Milestone-scoped questions
live in their milestone's `open_questions.md`. The rulings about the
process that a closed milestone left binding are here too, under their
ids ([§ The rulings about the process M1f left binding](#the-rulings-about-the-process-m1f-left-binding)).

Question IDs are **sticky** — when a question moves to a milestone
file it keeps its id; do not reuse a closed id.

## Index

| Q   | Title                                                                  | Lives in |
|-----|------------------------------------------------------------------------|----------|
| Q1  | What kind of graph is the IR? Typed multigraph, hypergraph, or e-graph? | here (cross-milestone) |
| Q2  | When does the graph engine hand off to a solver, and which one?         | here — **closed 2026-08-18: never** (M3 dropped) |
| Q3  | Surface IR syntax — heavy-semantic Lisp vs richer DSL?                  | here (decided in M1.P1.1 S1.1.1) |
| Q4  | Rule presentation language — host-language functions, graph-rewrite DSL, or Horn clauses? | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q5  | What "enough" means for the inference rule set                          | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q6  | Should symmetry breaking happen pre- or post-trace?                     | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q7  | Is the surface generator (NL output) allowed to be an LLM?              | [M2](m2_nl_to_ir/open_questions/README.md) |
| Q8  | Where do ambiguous NL parses go — branched on the IR, or rejected?      | [M2](m2_nl_to_ir/open_questions/README.md) |
| Q9  | Per-puzzle declared ontology vs ontology inferred from text             | [M2](m2_nl_to_ir/open_questions/README.md) |
| Q10 | When is direct LLM → constraint emission acceptable (no IR)?            | [M2](m2_nl_to_ir/open_questions/README.md) |
| Q11 | Does link-grammar enrich LLM input usefully, or is it dead weight?      | [M2](m2_nl_to_ir/open_questions/README.md) |
| Q12 | Which CT reading does the design commit to (A free / B schema / C functor)? | [followups F1](followups/f1_categorical_formulation.md) |
| Q13 | Self-modifying constraint language — feasibility + bounded problem      | [followups F2](followups/f2_self_modifying_language.md) |
| Q14 | Rule learning source — hand-written, library, LLM-suggested, or learned from walkthroughs? | [followups F4](followups/f4_cross_cutting.md) |
| Q22 | Ontology IR — three sub-heads (relations / definitions / types-and-objects)? | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q23 | What carries an explicit type slot in the data model?                         | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q24 | `:where` clause semantics in `sibling-exclusive`                              | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q25 | Cardinality + ordinality rules with vars — IR shape                          | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q26 | Compound / virtual node kinds for higher-order rules (sets, projections, …)  | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q27 | Relation declaration body form — bundled body vs separate property facts     | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q28 | Empty parens `()` — placeholder, ⊥, ⊤, or forbidden?                          | [M1](#absorbed--the-m1-milestone-questions-q4q42) |
| Q29 | Pattern: walker vs cached executable program — compile unit?                  | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20 |
| Q30 | Universal rule library + import mechanism — does `examples/rules.ein` exist?  | [M1](#absorbed--the-m1-milestone-questions-q4q42) — deferred to P1.8 (2026-05-20) |
| Q31 | `:why` template substitution language — `{?var}` or bare `{var}`?             | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20 |
| Q32 | `:where` semantics — sugar, top-level, or drop entirely?                      | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20; the pair refused since M1f S1f.5.11 (2026-11-09) |
| Q33 | Predicate primitives — minimal set for M1                                     | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20 |
| Q34 | Algebraic properties beyond symmetric/transitive + 2^7 cartesian product | [followups F4](followups/f4_cross_cutting.md) |
| Q35 | Variable typing via `(is-a ?var Type)` patterns                              | [followups F4](followups/f4_cross_cutting.md) |
| Q36 | Relation inheritance / rule polymorphism                                      | [followups F4](followups/f4_cross_cutting.md) |
| Q37 | Induction — rules from facts                                                  | [followups F4](followups/f4_cross_cutting.md) |
| Q38 | LLM as fact/relation/type/rule extractor                                      | [followups F4](followups/f4_cross_cutting.md) |
| Q40 | `(hypothesis ?h)` / `(contradiction-under ?h)` rule premises — Provenance map? | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20 |
| Q41 | Rule priority — scale (100/200/…) and placement (per-rule vs config)          | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20 |
| Q42 | P1.3 scope — path (a) narrow vs (b) all 10 with deferred                       | [M1](#absorbed--the-m1-milestone-questions-q4q42) — resolved 2026-05-20 |
| Q-M1e.19 | Algorithmic pathology has no owner: where does the search degrade?        | [here](#q-m1e19--algorithmic-pathology-has-no-owner) — moved from M1e's ledger at M1f S1f.20.8 (2026-11-17), id kept; open, no owner |
| Q-M1f.13 | F3's cause reaches a disjunction: whose row is it? | [here](#q-m1f13--f3s-cause-reaches-a-disjunction-whose-row-is-it) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted on the user's instruction: `W39/or-one-key` filed open and deferred to F22, which owns it; binds while the row is open |
| Q-M1f.51 | Three more tests whose slow half no CI tier runs | [here](#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs) — moved from M1f's ledger at its close (2026-11-25), id kept; taken by tier: every test's slow half runs in a tier (`nightly.yml`) |
| Q-M1f.53 | How the per-commit gate catches a register row the bundle cannot read | [here](#q-m1f53--how-the-per-commit-gate-catches-a-register-row-the-bundle-cannot-read) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (b): the review bundle's smoke builds a command case and reads every register row with no run |
| Q-M1f.55 | A `container` on a solve's `now` | [here](#q-m1f55--a-container-on-a-solves-now) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): a register case page runs the container, as `wrong_answers.rs` did |
| Q-M1f.77 | When is `utils/ci_container.sh` part of done? | [here](#q-m1f77--when-is-utilsci_containersh-part-of-done) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): a stage that changes a DOT writer, the review bundle or what CI installs is not done until `utils/ci_container.sh` is green on its commit (AGENTS.md) |
| Q-M1f.78 | Where the runner's missing `fdp` is fixed | [here](#q-m1f78--where-the-runners-missing-fdp-is-fixed) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): the runner installs `libgvplugin-neato-layout8`, and `run_tests.sh` exits 127 where `fdp` lays out no graph |
| Q-M1f.79 | Does a refused DOT fail the whole bundle? | [here](#q-m1f79--does-a-refused-dot-fail-the-whole-bundle) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): a DOT Graphviz refuses fails the whole review bundle, as in the smoke |
| Q-M1f.102 | What the CI nightly keeps, and what the dev host runs | [here](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): the nightly keeps what guards an answer or a contract no per-commit step guards, each step within 5 min on the runner; the dev host runs the rest (AGENTS.md) |
| Q-M1f.103 | The fuzzer's budget in the CI nightly | [here](#q-m1f103--the-fuzzers-budget-in-the-ci-nightly) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): the nightly's fuzzer, 20 000 inputs a seed; the dev host's, 1 000 000 |
| Q-M1f.105 | How the dev host's list is held | [here](#q-m1f105--how-the-dev-hosts-list-is-held) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): the dev host's list is a superset of the nightly's, every other step named in `DEV_HOST_ONLY` with why |
| Q-M1f.106 | The whole review bundle on the runner's Graphviz | [here](#q-m1f106--the-whole-review-bundle-on-the-runners-graphviz) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): the whole review bundle runs on the dev host, here or in the runner's image |
| Q-M1f.134 | What a register row records as `right` while F24's question is not ruled | [here](#q-m1f134--what-a-register-row-records-as-right-while-f24s-question-is-not-ruled) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted: a register row records `right` under one named reading of F24's question, each other reading an alternative (F24, F21) |
| Q-M1f.150 | The counter, and how the measured test is in the gate | [here](#q-m1f150--the-counter-and-how-the-measured-test-is-in-the-gate) — moved from M1f's ledger at its close (2026-11-25), id kept; overruled by the user: the calculator's batch is out of the gate, run by hand (AGENTS.md) |
| Q-M1f.284 | The worker pool's count, its heavy lane and its memory bound | [here](#q-m1f284--the-worker-pools-count-its-heavy-lane-and-its-memory-bound) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): the corpus sweeps' pool, 16 workers, a quarter of them the heavy lane (AGENTS.md) |
| Q-M1f.285 | The nightly's and the dev host's test steps on the gate profile | [here](#q-m1f285--the-nightlys-and-the-dev-hosts-test-steps-on-the-gate-profile) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): every test step of every tier on the gate profile (AGENTS.md) |
| Q-M1f.287 | One pass for `corpus_shapes` and `id_order_invariance` | [here](#q-m1f287--one-pass-for-corpus_shapes-and-id_order_invariance) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a): one pass for `corpus_shapes` and `id_order_invariance` (AGENTS.md) |
| Q-M1f.288 | `jobs_invariance` over the programs that search | [here](#q-m1f288--jobs_invariance-over-the-programs-that-search) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a): `jobs_invariance` per commit over the programs that search (AGENTS.md) |
| Q-M1f.289 | `id_order_invariance` over the programs that search | [here](#q-m1f289--id_order_invariance-over-the-programs-that-search) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a): `id_order_invariance` permutes per commit the programs that search, the full corpus in the nightly (AGENTS.md) |
| Q-M1f.290 | `tests/` out of the render and determinism sweeps | [here](#q-m1f290--tests-out-of-the-render-and-determinism-sweeps) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a): `tests/` out of the per-commit render and determinism sweeps (AGENTS.md) |
| Q-M1f.291 | The five sweeps that set modes and routes against one another | [here](#q-m1f291--the-five-sweeps-that-set-modes-and-routes-against-one-another) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a): the five sweeps that set modes and routes against one another kept, a `slow` entry in the slow half (AGENTS.md) |
| Q-M1f.292 | One load a program, shared by the outputs that do not change it | [here](#q-m1f292--one-load-a-program-shared-by-the-outputs-that-do-not-change-it) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a): one load a program, cloned for each output (AGENTS.md) |
| Q-M1f.293 | The nightly's steps against five minutes | [here](#q-m1f293--the-nightlys-steps-against-five-minutes) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled: a nightly step over five minutes on the runner runs in shards, staying in CI (AGENTS.md, `nightly.yml`) |
| Q-M1f.294 | `answer_dump`'s `heavy_dump` witness, and the feature tests the pool does not reach | [here](#q-m1f294--answer_dumps-heavy_dump-witness-and-the-feature-tests-the-pool-does-not-reach) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (a) and (a): `answer_dump`'s `heavy_dump` witness in the nightly, and the feature tests the pool does not reach (`nightly.yml`) |
| Q-M1f.296 | The bit tables, by the digit tables' generator | [here](#q-m1f296--the-bit-tables-by-the-digit-tables-generator) — moved from M1f's ledger at its close (2026-11-25), id kept; defaulted (a): `utils/gen_arith_tables.py` writes the digit and the bit tables, one `--check` |
| Q-M1f.304 | `route_invariance` past the default budget, past any shard's five minutes | [here](#q-m1f304--route_invariance-past-the-default-budget-past-any-shards-five-minutes) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled (d): no test runs past the default `--region-budget` (AGENTS.md, `nightly.yml`, `run_tests.sh`) |
| Q-M1f.314 | What the release-voice check reads, and what fails a line | [here](#q-m1f314--what-the-release-voice-check-reads-and-what-fails-a-line) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled as defaulted: what the release-voice check reads, and what fails a line (`utils/check_release_voice.py`) |
| Q-M1f.496 | The `fdp` provisioning, now that no view suggests `fdp` | [here](#q-m1f496--the-fdp-provisioning-now-that-no-view-suggests-fdp) — moved from M1f's ledger at its close (2026-11-25), id kept; ruled, the default: the `fdp` provisioning left as it is (AGENTS.md's Graphviz paragraph) |

---

## Q1 — What kind of graph is the IR?

**Decision file:** [`docs/decisions/language/Q1-the-knowledge-base-is-a-hypergraph-of-atoms.md`](../docs/decisions/language/Q1-the-knowledge-base-is-a-hypergraph-of-atoms.md)

Per [idea 02 §Open questions](ideas/02-graph-as-formal-substrate.md)
the candidates are: plain directed graph, typed multigraph, typed
hypergraph (relations span >2 vars), adhesive category, e-graph on
top. The 2021 prototype is a typed digraph with set-valued edges; it
cannot natively express `next_to(a, b, c)`-style ternary or `allDifferent`
constraints without faking them.

**Working answer**: **typed hypergraph with provenance and equality
class IDs**. Hyperedges accommodate global constraints; equality-class
IDs leave the door open to e-graph promotion if equality saturation
becomes relevant (per [idea 06 row 2](ideas/06-inference-rules-completeness.md)).
The decision is finalised in M1.P1.2 S1.2.1.

## Q2 — When does the graph engine hand off?

**Decision file:** [`docs/decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md`](../docs/decisions/engine/Q2-ein-hands-no-problem-to-an-external-solver.md)

[Idea 02 §When does the graph stop being enough](ideas/02-graph-as-formal-substrate.md)
names three threshold conditions: arithmetic-dominated constraints,
large combinatorial search, and global cardinality where a real
solver dominates. The split is *not* "graph-engine for everything
small, solver for everything large" — it's per-sub-problem.

**Closed 2026-08-18 — it doesn't.** M1 shipped a pure graph-native
engine, and M3 (which would have introduced SMT-LIB emission for
`(hard-slice …)` slices, with a pure-graph / hybrid / solver-first mode
flag) was dropped as an idea, not just descheduled. There is no handoff
and no back-end to pick. Idea 02's threshold conditions
(arithmetic-dominated constraints, large combinatorial search, global
cardinality) still describe where a graph-native engine is *weakest* —
they are now a statement about Ein's scope rather than a trigger for a
solver call, and they are the honest limitations paragraph
[M5](m5_presentation/README.md) Track A owes its readers.

## Q3 — Surface IR syntax

**Decision file:** [`docs/decisions/language/Q3-ein-is-an-s-expression-language.md`](../docs/decisions/language/Q3-ein-is-an-s-expression-language.md)

Decided in M1.P1.1 S1.1.1. The leading candidate is a small
S-expression dialect with named atoms carrying most of the semantic
load (per [idea 01 §Open questions point 3](ideas/01-self-modifying-constraint-language.md)
recommendation), so a future GBNF grammar (M2 [S2.2.3](m2_nl_to_ir/p2.2_formalizer/s2.2.3_gbnf.md)) is trivial.
Counter-arguments and richer-DSL alternatives are listed there.

---

# Absorbed — the M1 milestone questions (Q4–Q42)

M1 shipped 2026-06-17 and its plan folder was deleted in P1.22
(S1.22.99). Its `open_questions.md` is merged below rather
than dropped: `docs/kernel/` deep-links thirteen of these anchors — the
*resolved* ones as much as the open ones, because a resolved question is
the record of the decision. Ids are sticky, so every `#q<n>` anchor still
resolves; only the file changed. Links that pointed into the deleted
folder are now plain text — the stage id is the citation.

**Still open:** Q26 (compound / virtual node kinds) and Q28 (empty-parens
semantics). Everything else below is closed and kept as the record.

## Q4 — Rule presentation language

**Decision file:** [`docs/decisions/language/Q4-a-rule-is-a-pattern-a-conclusion-and-a-reason.md`](../docs/decisions/language/Q4-a-rule-is-a-pattern-a-conclusion-and-a-reason.md)

Per [idea 06 §Open sub-questions point 1](ideas/06-inference-rules-completeness.md).

**Options:**

- **A** — Host-language predicate over a `Graph` view. Easy; awful for
  trace generation (the rule "fires" with no structured premise list).
- **B** — Graph-rewrite DSL (LHS pattern → RHS pattern, à la DPO).
  Structural, formal, slightly heavy. Matches
  [idea 07 Reading C](ideas/07-categorical-formulation.md).
- **C** — Horn-clause / Datalog encoding. Familiar declarative;
  loses the "graph rewrite" intuition.

**Working answer**: B — a small DSL where each rule is
`(rule <name> :match <pattern> :assert <conclusion> :why <reason-template>)`.
The original sketch carved out an engine-code fallback for "cases
where a pattern can't express what we need", citing the spatial
constraint of P1.4 as the trigger; the declarative square rule
(see [Q17](#q17--spatial-relation-formalisation)) removes that
trigger. Keep the engine-code fallback latent as an *escape hatch* for
future problem classes, but Zebra ships without using it.
Locked in P1.3 S1.3.1.

## Q5 — What "enough" means

Per [idea 06 §What "enough" should mean](ideas/06-inference-rules-completeness.md):
four non-equivalent answers — functional, propagation, explanation,
domain.

**Working answer**: M1's target is **explanation-complete on Zebra**
(every step of the human walkthrough in
[idea 08](ideas/08-human-style-deductive-trace.md) maps to
a named rule firing). Propagation-completeness is desirable but not
required to ship. Domain-completeness is out of scope; revisit per
problem class.

## Q6 — Symmetry breaking

**Decision file:** [`docs/decisions/answer/Q6-symmetry-is-the-programs-not-the-engines.md`](../docs/decisions/answer/Q6-symmetry-is-the-programs-not-the-engines.md)

Per [idea 08 §Open questions point 3](ideas/08-human-style-deductive-trace.md).
Two paths initially considered:
(a) pre-trace — engine enforces a canonicalisation (e.g. house
numbering); (b) post-trace — engine explores both, trace planner
annotates "doesn't matter which way numbered".

**Resolved 2026-05-21** (user direction during S1.5.0 review):
**neither (a) nor (b)** — the question doesn't belong to the
engine. By the time a `.ein` file declares `House-1`, …, `House-5`
plus `(right-of House-2 House-1)` etc., the orientation is *already
committed* by construction. The engine sees unique named instances
with one spatial layout; there is no second orientation to explore
or canonicalise.

The "WLOG numbering" remark in idea 08's target trace documents
the **ontology writer's** choice, not a derivation step. M1's
hypothesis loop (P1.5) does no symmetry handling. M2's NL → IR
pipeline inherits the decision: the NL parser maps positional
phrases ("the first house", "the leftmost") to named House
instances by adopting a convention.

S1.5.3 (the original "symmetry detection placeholder" stage) is
**dropped** from M1 — see S1.5.0 review §D.

Future puzzles with *latent* symmetry that can't be committed at
ontology-write time (e.g. graph automorphism on an abstract group)
remain a research direction — beyond M1's scope.

## Q15 — Rule ordering

Per [idea 06 §Open sub-questions point 3](ideas/06-inference-rules-completeness.md).

**Working answer**: forward-chaining queue with a static priority
class per rule (cheap propagation rules fire before expensive
search-like ones); revisit only if the trace shows obviously bad
ordering. Decided in P1.3 S1.3.3.

## Q16 — Universal vs per-puzzle rules

**Decision file:** [`docs/decisions/stdlib/Q16-general-rule-families-are-stdlib-modules.md`](../docs/decisions/stdlib/Q16-general-rule-families-are-stdlib-modules.md)

Per [idea 06 §Open sub-questions point 4](ideas/06-inference-rules-completeness.md).

**Working answer**: ship with a *universal core* (composition,
equality, exclusivity, exhaustion, hypothesis-contradiction,
arc-consistency, cardinality, forced-unique). Problem-specific
rules — including the *square rule* (formerly the "spatial bundle"
loaded by engine code on `:spatial-via`) — are now ordinary rule
declarations + property-application facts inside the puzzle's `.ein`
file. No bundle-loading mechanism; the IR is the only configuration
surface. New problem classes write their own rules in their own
files, same as Zebra. Decided in P1.3 + P1.7.

The `:spatial-via SpatialAttribute` kw-pair was the original
mechanism for the spatial bundle; resolved on 2026-05-18 by the
declarative answer to [Q17](#q17--spatial-relation-formalisation).

## Q17 — Spatial-relation formalisation

**Decision file:** [`docs/decisions/language/Q17-spatial-structure-is-facts-and-rules-with-no.md`](../docs/decisions/language/Q17-spatial-structure-is-facts-and-rules-with-no.md)

The 2021 prototype's open question
([idea 05 §Open question recorded in the README](ideas/05-zebra-puzzle-graph-reasoner.md#open-question-recorded-in-the-readme)).

**Options:**

- Allen interval algebra (13 named relations between intervals).
- RCC (region connection calculus).
- Ad-hoc 1-D position lattice with `pos(x) ∈ {1..N}` and integer
  arithmetic on `pos`.
- **Declarative graph-only:** materialise the spatial structure as
  pairwise `(right-of House_{i+1} House_i)` facts; reuse the 2021
  prototype's *square rule* (now `square-fwd` / `square-bwd` rules + property
  facts) to project spatial relations across `co-located`. No
  integer arithmetic; no position lattice; no `:spatial-via` engine
  hardcode.

**Working answer**: the declarative graph-only formulation. Decided
2026-05-18 in this conversation; locks the answer for Zebra. Pulls
Q17 forward from P1.4 to S1.1.1.

Implementation already shipped in `examples/zebra.ein`:

- Drop the `SpatialAttribute` type and the `:spatial-via` kw-pair on
  `right-of` / `next-to`.
- Add `square-fwd` and `square-bwd` rules (gated by
  `(square-fwd ?R)` / `(square-bwd ?R)` rule-application facts on
  `right-of` and `next-to`).
- Materialise condition (1) "five houses in a row" as four
  `(right-of House_{i+1} House_i)` facts; `(implies right-of next-to)`
  + `(symmetric next-to)` derive the eight next-to pairs.

Trade-offs:

- *For:* IR-native, no engine-code fallback. The trace renderer reads each
  spatial step as a square-rule firing — fully visible to
  [idea 08](ideas/08-human-style-deductive-trace.md). (The original
  entry also noted that an SMT translation would map `square-fwd` /
  `square-bwd` to two universally-quantified clauses with no integer
  theory required — moot since M3 was dropped 2026-08-18, but it is
  still evidence that the encoding is a *logical* one rather than an
  arithmetic trick.)
- *Against:* problems with continuous or multi-dimensional spatial
  structure (Allen, RCC) will need a different formulation. Revisit
  for those problem classes; for Zebra-class puzzles the graph-only
  answer is sufficient.

P1.4 S1.4.2 keeps the *open question for richer problem classes*
(when does the graph-only answer break?) but no longer ships an
integer-arithmetic spatial encoding for Zebra.

## Q18 — Provenance granularity

**Decision file:** [`docs/decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md`](../docs/decisions/engine/Q18-provenance-is-per-fact-an-or-of-derivations-over.md)

Per [idea 03 §What "contradictions" specifically means](ideas/03-three-task-classes.md)
and [idea 08](ideas/08-human-style-deductive-trace.md).

**Working answer**: per-derived-edge provenance — each edge carries
a tuple `(rule, premise_edges, source_or_rule_id)`. The full
derivation DAG is recoverable by transitive closure.

**Decided** in P1.2 S1.2.3 (2026-05-19): implemented as the
`Provenance` dataclass (`kind in {'source', 'rule', 'hypothesis',
'rejected'}` + the discriminator-specific fields). Each `Fact`
carries an optional `provenance: Provenance | None`. The full DAG
is built by `KnowledgeBase.derivation_dag(fact)` via BFS over
`provenance.premises_raw`. `KnowledgeBase.unsat_core(facts)` returns
the minimal source-kind frontier — the input to the *contradictions*
task class.

IR-level round-trip of `:using` is deferred (the current grammar
doesn't accept headless lists of compact-form fact refs) — see
S1.2.3 T1.2.3.4.

## Q19 — Hypothesis branching strategy

**Decision file:** [`docs/decisions/hypotheses/Q19-hypotheses-are-proposed-at-a-fixpoint-after.md`](../docs/decisions/hypotheses/Q19-hypotheses-are-proposed-at-a-fixpoint-after.md)

The 2021 prototype's algorithm branches whenever `solved(s)` returns
false. Modern CDCL solvers branch only when propagation saturates.

**Working answer**: lazy branching — saturate first with all
propagation rules; branch only when no rule fires and the graph is
still incomplete. Mirrors human walkthroughs. Decided in P1.5 S1.5.1.

## Q20 — Trace reordering

**Decision file:** [`docs/decisions/read-out/Q20-the-trace-is-in-the-engines-order-and-reorder.md`](../docs/decisions/read-out/Q20-the-trace-is-in-the-engines-order-and-reorder.md)

Per [idea 08 §Why this is a hard problem — Ordering](ideas/08-human-style-deductive-trace.md).

**Working answer**: emit engine-order trace + a `--reorder` flag
that runs a planner pass clustering by entity. Defer full
human-template fitting to M2 (where an LLM surface generator is
available). Decided in P1.6 S1.6.4.

## Q21 — IR ↔ DOT structural isomorphism

**Decision file:** [`docs/decisions/cli/Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md`](../docs/decisions/cli/Q21-every-form-has-a-fixed-dot-shape-and-its-layout.md)

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
  [Q1](#q1--what-kind-of-graph-is-the-ir)'s typed-hypergraph + equality-class-ID
  answer visually.
- Rule rendering: 3 modes (side-by-side / DPO span / overlay),
  configurable; `rules.ein` defaults to side-by-side, traces default
  to overlay.
- Trace rendering: 3 views (per-step / aggregate / derivation-DAG),
  configurable; default per-step (matches M1 acceptance §2).
- Branch rendering: tree-of-states for the search-tree view,
  sub-clusters per branch for per-state snapshots.

Decided in S1.1.1 T1.1.1.6.

## Q22 — Ontology IR sub-head split

Currently one `(ontology …)` block holds: type declarations,
instance declarations, relation signatures, rule-application facts
(`(symmetric R)` etc.), and structural facts (e.g. the four
`(right-of House_{i+1} House_i)` in zebra.ein). The KB loader
(P1.2 S1.2.1 T1.2.1.4) already routes these to different slots
internally.

The user proposed (2026-05-18) splitting the `(ontology …)` block
*syntactically* into three sub-heads:

```lisp
(ontology
  (relations
    (relation co-located (Attribute Attribute))
    (relation right-of   (Attribute Attribute))
    …)
  (definitions               ;; rules + rule-application facts
    (symmetric co-located)
    (transitive co-located)
    (square-fwd right-of)
    …)
  (types-and-objects
    (type Attribute) …
    (instance Norwegian Nationality) …
    (instance House-1 House) …))
```

Justification (user's): "rules are separate head — good. Ontology
can be split into: relations declarations, relations definitions
(via rules), types looks secondary as produced by a relation, so
types and objects go into 3rd separate ontology sub-head."

**Working answer:** *parked*. The data model is already split
internally; the syntactic re-org is a cosmetic IR change that does
not affect P1.2's data-model decisions. Revisit during P1.7
authoring when the larger Zebra IR is being read by multiple
people — readability may justify the split there.

**Counter-argument:** the current flat (ontology …) block reads
fine for Zebra-scale puzzles (one page). Premature factoring; wait
for an instance where it bites.

**Update (2026-06-17) — moot.** P1.7c removed the `(ontology …)` /
`(facts …)` / `(rules …)` block heads entirely (flat-form grammar,
S1.7c.1/.4): a program is now a flat
sequence of forms classified by head, layer attributed per fact. There is no
longer an `(ontology …)` block to split into sub-heads, so this question is
**closed as superseded**.

## Q23 — What carries an explicit type slot

**Decision file:** [`docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md`](../docs/decisions/language/Q23-a-premise-says-what-a-variable-ranges-over.md)

Direct question raised 2026-05-18: which entities in the data model
have a `type` slot?

**Working answer** (locked in P1.2 S1.2.1):
- `Instance.type` — yes, exactly one. Every instance belongs to one
  declared type (no multi-typing in M1).
- `Var` — *no* explicit slot. Variables are typed *structurally* by
  the premises that bind them (`(is-a ?var T)` etc.); see
  [F4 Q35](followups/f4_cross_cutting.md#variable-typing-via-match-is-a-var-type-q35).
- `Relation` — *no* explicit slot, but has a `signature: tuple[Type,
  …]` that says what types its arguments can be. Not the same as
  "the relation has a type" in the OOP sense.
- `Type` / `Rule` / `Fact` — no type slot. (Types form a hierarchy
  via `parent`; rules and facts are not themselves typed.)

This consciously aligns with the unified `is-a` model of
zebra2.ein: types are graph nodes, "having a type" is "being on the
receiving end of an `is-a` / `instance-of` edge". The slot exists
only as a fast-access cache on `Instance`.

**Multi-typing** (user's question): out of M1 scope. If an instance
must be of two types, declare an intermediate intersection type or
use additional `(is-a)` facts in the unified model. Revisit if the
need is real.

**Update (2026-06-17) — answer partly stale.** The *decision* stands (vars
typed by premises, not a slot; relations carry a `signature`, not a type), but
the `Instance.type` slot named above **no longer exists**: the `Type` /
`Instance` entity-view was retired in
S1.7.23
(atoms-vs-objects unification). Type membership is now plain `is-a` facts over
ordinary atoms; no entity caches a type slot.

## Q24 — `:where` clause in `sibling-exclusive`

Surfaced 2026-05-18. The clause appears in some early rule sketches
but is *not* documented in the rule DSL spec (P1.3 S1.3.1). The
options:

- **A** — `:where` is a top-level rule clause holding side
  conditions evaluated against the matcher's bindings (similar to
  Prolog cut + guard). Currently used inline inside `(and …)`
  patterns; pulling it to top-level is a cosmetic move.
- **B** — `:where` is the same as `(and … :where p …)` inline form,
  just a sugar; nothing to add to the DSL.
- **C** — delete the keyword everywhere; require all side
  conditions to be ordinary pattern premises.

**Working answer:** B — it's syntactic sugar for the inline
`:where` predicate inside an `(and …)`. Document in P1.3 S1.3.1
DSL spec; do not promote to top-level keyword. Locks when the DSL
is written down.

## Q25 — Cardinality and ordinality rules with vars

**Decision file:** [`docs/decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md`](../docs/decisions/stdlib/Q25-ein-has-no-ordinal-form-an-order-is-a-relation.md)

The rule library has `global-cardinality` (`exactly-n X V N`,
T1.3.2.8) — counting facts with a value. The user (2026-05-18)
asked whether **ordinality rules** (less-than, position-of, between)
should be first-class IR forms or derived from `(rel ?r ?a ?b)`
patterns over a chosen relation.

**Options:**

- **A** — first-class `(less-than ?a ?b)` form + a dedicated rule
  family. Tight coupling to the position lattice (P1.4 S1.4.2).
- **B** — derived from `right-of` / `before` etc. via the existing
  square + transitivity rules. No new IR forms; everything reduces
  to the generic relation machinery.
- **C** — half-and-half: keep ordinality as a *named property* (a
  rule-application fact `(ordinal ?R)`) that activates a small
  bundle of derived rules.

**Working answer:** B for Zebra. Ordinality questions ("is house 3
to the right of house 1?") reduce to transitive closure over
`right-of`. C may be needed for Sudoku-style puzzles where ordinal
position matters arithmetically. Revisit in P1.3 S1.3.2 with the
demo-problem set — if the cardinality/ordinality demos need an
ordinal-specific rule, promote to a named family.

## Q26 — Compound / virtual node kinds for higher-order rules

User direction (2026-05-18, before P1.2 begins): the data model
should support not only the base kinds (Type, Instance, Relation,
Rule, Fact) but eventually a class of *compound* / *virtual* node
kinds that higher-order rules will need. Examples:

- **Relation-set-of-object** — the set of all relations involving a
  given object.
- **Slot-projection** — all objects connected to argument-slot N of
  a given relation (e.g. "all 'subjects' of `is-a`").
- **Top / bottom of relation subgraph** — the maximal / minimal
  element under a relation (when applicable, e.g. `is-a` lattice).
- **Criterion-selected group** — objects matching a graph pattern
  predicate (e.g. zebra.ein's three middle houses = those with
  exactly two `next-to` neighbours).
- "*Who knows what else…*" — open class.

These are not pre-stored entities; they are *computed-on-demand
subgraphs* materialised when a higher-order rule binds a parameter
to one.

**Options:**

- **A** — Implement nothing in M1. Concrete kinds land as needed in
  followups; P1.2 just doesn't foreclose them.
- **B** — Reserve a `VirtualNode` base class / kind tag in the data
  model. M1 doesn't subclass it; the type hierarchy exists for
  later extension.
- **C** — Implement one (say, slot-projection) as a worked example
  in M1; defer the rest.

**Working answer:** A — *don't foreclose*. P1.2 ships the base
entity kinds (Type, Instance, Relation, Rule, Fact); the store and
the index design must accept new kinds without rework, but no
specific kind is implemented in M1. When the first higher-order
rule actually *needs* one (likely in F4 promotion work), promote
the kind to a concrete entity at that time. Documented for design
review; concrete promotion happens followup-by-followup.

Connection: feedback memory `graph-canonical` — graph is the
canonical data model, entity API is a derived view; this open
question is the *future-proofing* corollary.

## Q27 — Relation body form

**Decision file:** [`docs/decisions/language/Q27-a-relations-properties-are-facts-beside.md`](../docs/decisions/language/Q27-a-relations-properties-are-facts-beside.md)

User direction (2026-05-19): two equivalent forms for declaring a
relation along with its algebraic properties:

```lisp
;; Form (a) — body bundled into the declaration
(relation is-a (A B) (transitive asymmetric sibling-exclusive))

;; Form (b) — declaration + separate property-application facts (M1)
(relation is-a (T T))
(transitive        is-a)
(asymmetric        is-a)
(sibling-exclusive is-a)
```

Semantically equivalent. Which form, and why? See
[`docs/ein/user/03-syntax-and-semantics/04-ontology.md` §Two forms that look alike](../docs/ein/user/03-syntax-and-semantics/04-ontology.md#two-forms-that-look-alike)
for the full trade-off table.

**Working answer:** ship form **(b)** in M1 (already the de-facto
form in zebra.ein / zebra2.ein). Form (a) is admitted as a *future
syntactic sugar* that desugars into form (b) at load time; it does
NOT need first-class engine support. The desugaring lands when
authoring readability becomes a pain point (probably P1.7+ when
non-zebra puzzles arrive).

Trade-off rationale (from `03_ein_model.md` §7.2):
- Form (b) makes property-application facts *first-class graph
  nodes*, so rules (T2 + F5) can match and modify them.
- Form (a) is more readable in the small but hides the property
  facts inside a nested form; rules wanting to match them would need
  unpacking.
- Choosing (b) aligns with the "everything is a node" principle
  (feedback memory `graph-canonical`).

## Q28 — Empty parens `()` node semantics

User direction (2026-05-19): the grammar accepts `()` as a parenth-
esised form; what does it *mean*?

Candidates:
- **Placeholder / hole** — "something belongs here but is
  undetermined". Useful in partially-derived facts.
- **Global singleton ⊥ (bottom)** — the impossible / empty type.
- **Global singleton ⊤ (top)** — the universal-supertype.
  (zebra2.ein uses an explicit `T` atom for this — `()` would be
  the unnamed equivalent.)
- **Forbidden** — `()` is a parse error; no hole notation.

**Working answer:** *parked*. The grammar's current behaviour
parses `()` as a synthetic `#empty`-headed `SForm` (`@empty` until
M1f S1f.5.5's T1f.5.5.8, 2026-11-08) with no
attributes; the engine treats it as a no-op atom. No semantic
commitment in M1. Revisit when a concrete use case forces the
question — most likely the negative-fact representation in P1.5
(hypothesis loop) or the partially-derived-fact representation in
P1.6 (trace).

When a use case arrives, the *placeholder / hole* reading is the
most likely choice because:
- ⊥ and ⊤ are better served by explicit named atoms (`T` in
  zebra2.ein is already the convention).
- *Forbidden* would require grammar work and offers no upside.

## Q29 — Pattern: walker vs cached executable program

**Decision file:** [`docs/decisions/engine/Q29-a-rule-compiles-once-per-activator.md`](../docs/decisions/engine/Q29-a-rule-compiles-once-per-activator.md)

Surfaced 2026-05-19 during the P1.3 review. P1.2 ships
`kb.Pattern` as a structural lift of the rule's `:match` / `:assert`
IR clause — `expr: IRNode` plus pre-computed metadata (variables,
relation_names, type_names, has_instance_pattern). The original
P1.3 plan proposed a *separate* `Pattern` AST inside `rules/`.

**Options:**

- **A — Walker**: matcher walks `kb.Pattern.expr` directly,
  dispatching on `IRNode` subtype each step.
- **B — Cached executable program** in `inference/`: lazy compile
  to opcodes (`Scan`, `Join`, `Guard`), cached by `Pattern.expr`
  identity.
- **C — Extend `kb.Pattern`** with a `compiled` field.

**Resolved 2026-05-20** — option **B**, with the **compile unit
being per (rule, activator-binding) pair** (not per `Pattern`
alone). The activator fact (e.g. `(transitive co-located)`) binds
the rule's parameter list *before* matching begins; the compiler
substitutes the parameters and bakes concrete relation names into
the program. Cache key: `(rule.name, activator.args)`. Zebra.ein
has 6 activators → 6 compiled programs.

Option A rejected: a walker re-discovers the activator binding on
every match attempt — wrong granularity for activator-driven T2
polymorphism. Option C rejected: contaminates kb with engine state
(inverts the kb → inference dependency direction).

Full analysis:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q29.

## Q30 — Universal rule library + import mechanism

Surfaced 2026-05-19 during the P1.3 review.

**Options:**

- (a) `examples/rules.ein` is a universal library every puzzle
  imports.
- (b) rules are always inline in the puzzle's `.ein` file (current
  zebra.ein).
- (c) hybrid — a kernel core auto-loaded, puzzles can extend.

**Deferred to P1.8** (user direction 2026-05-20). The universal
rule library question entangles with an *import / include*
mechanism in the IR language (how one `.ein` file references rules
in another), which is too large for P1.3 to resolve and not on the
M1-acceptance critical path. P1.8 — a new M1 phase, name + slot
TBD — will design the library + import mechanism and decide
between (a) / (b) / (c).

For P1.3: assume zebra.ein-style **inline rules per puzzle**
(option b's de-facto form).

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q30.

## Q31 — `:why` template substitution language

**Decision file:** [`docs/decisions/language/Q31-a-why-template-refers-to-a-variable-as-var.md`](../docs/decisions/language/Q31-a-why-template-refers-to-a-variable-as-var.md)

Surfaced 2026-05-19 during the P1.3 review. Zebra.ein uses
`{?rel}` (ein-var style, `?` prefix). The original P1.3 plan used
`{r}` / `{p0}` (bare-name format fields).

**Resolved 2026-05-20** — adopt the `{?var}` notation. The `?` is
*part of the reference*, not a prefix that's stripped; `?var` is
simultaneously the node name and the var name. Inside the `:why`
string, `{?rel}` substitutes to the bound node name (e.g.
`co-located`), `{?a}` to the bound argument (e.g. `Norwegian`),
etc. The `?` is preserved through binding — there is no separate
"bare var name" form.

Grammar: the template tokeniser scans for `{` … `}` delimited
sequences whose body starts with `?` followed by a var-name. Any
`{x}` *without* a leading `?` is a literal `{x}` (not substituted).

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q31.

## Q32 — `:where` semantics

Surfaced 2026-05-19 during the P1.3 review. Q24's working answer
was "`:where` is sugar for inline `(and …)` premise"; zebra.ein's
`transitive` rule uses `:where (neq ?a ?c)` *as a top-level
keyword inside `(and …)`* — already a top-level construct.

**Resolved 2026-05-20** — **drop `:where`; ship a built-in
predicate registry**. All four
candidate benefits of `:where` over canonical `(and …)` (guard-
vs-generator distinction, prevent auto-vivification, readability,
future-proofing) dissolve once a predicate registry exists; the
registry is strictly more general.

Concrete:
- The registry — initial set `eq`, `neq` (per Q33).
- The loader consults the registry before auto-
  vivifying unknown heads.
- Matcher dispatches premises via `(head ∈ registry)` → guard,
  else fact-scan.
- Migration: rewrite zebra.ein's `transitive` rule from
  `:where (neq ?a ?c)` → positional `(neq ?a ?c)`.

Closes Q24 retroactively (the "sugar" framing was effectively
saying "drop the keyword once registry exists").

**The pair is refused since M1f S1f.5.11 (2026-11-09).** Q32 removed
`:where` and left a pair written anyway to be dropped, by the compiler, in
silence, so the rule fired unguarded. The user's ruling of 2026-11-07
([Q-M1e2.28](../docs/decisions/language/Q-M1e2.28-each-form-takes-a-closed-list-of-keywords.md),
the no-owner inventory's D-15 and G-22) supersedes that half: each form takes a
fixed set of keywords and a term takes none, so `:where` is refused at the
parse wherever it is written since S1f.5.5's swap (2026-11-09), and
S1f.5.11's
T1f.5.11.2 retired the compiler's drop. The positional guard, `(neq ?a ?c)`,
is what Q32 asked for and what every program writes.

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q32.

## Q33 — Predicate primitives — minimal set for M1

Surfaced 2026-05-19 during the P1.3 review. The original P1.3 plan
listed `forall`, `exists`, `count`, `single`, `the`, `\` (set
diff), `in`, variadic `?xs...` as needed primitives for T1.3.2.3
/ .4 / .8 / .9.

**Resolved 2026-05-20** (user direction): M1 ships **only the
minimal predicate set** required for the current 6 rules.

### M1 built-in predicate registry

```text
eq   ?a ?b   — the two resolved arguments are equal
neq  ?a ?b   — the two resolved arguments differ
```

Two binary predicates. That's it.

### What's not a predicate (and why)

- `(rel ?a ?b)` pattern matching — matcher's core operation;
  existence check is pattern matching itself.
- `(not (rel ?a ?b))` — structural wrapper (matcher: negation as
  failure; asserter: negative fact assertion). Not a registry entry.
- `forall ?x P(?x)` — already `:match (P(?x))`; the clause is
  implicitly universally quantified.
- `exists ?x P(?x)` — already `:assert (P(?x))`; adding a fact IS
  the existential.
- `single ?x P(?x)` — derivable: `(and P(?x) (not (and P(?y) (neq ?y ?x))))`.

### Deferred to followups (not P1.4, not P1.7)

- Variadic args `?xs...`
- Numeric predicates: `lt`, `gt`, `le`, `ge`
- Set operations + aggregation: `count`, `member`, `in`, set diff
- Domain primitives: `dom`, `functional`

A future F-numbered followup adjacent to F4 owns these.

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q33.

## Q40 — Hypothesis-rule premises

**Decision file:** [`docs/decisions/language/Q40-a-fact-may-hold-a-fact-as-an-argument.md`](../docs/decisions/language/Q40-a-fact-may-hold-a-fact-as-an-argument.md)

Surfaced 2026-05-19 during the P1.3 review. The original P1.3
plan's T1.3.2.5 sketched:

```lisp
(rule hypothesis-contradiction
  :match  (and (hypothesis ?h) (contradiction-under ?h))
  :assert (not ?h)
  ...)
```

The premises `(hypothesis ?h)` and `(contradiction-under ?h)`
don't correspond to any fact in the shipped data model — P1.2's
`Provenance` carries `kind='hypothesis' | 'rejected'` on the fact
itself, not as a separate fact.

**Options:**

- **A — Synthetic facts**: engine emits `(hypothesis <fact>)` and
  `(contradiction-under <fact>)` as auto-vivified relations; the
  matcher dispatches normally.
- **B — Engine procedure**: hypothesis-contradiction is a special
  engine procedure that doesn't go through the matcher.

**Resolved 2026-05-20** — option **A**. The kernel ein model
(`docs/kernel/ir/01-ein-graph/03_ein_model.md` §3) already says
facts can have **relational nodes** as args — `(hypothesis
(co-located Norwegian House-2))` is just a fact whose first arg
is a relational node. This is *not* a compound-node-kind promotion
(Q26 is about virtual / computed kinds; different concern); it's
the kernel's existing node-as-arg duality.

Required work:
- **`Fact.args` widening**: `tuple[str | int | Fact, ...]`. Small
  P1.2 patch.
- **Loader recursion** for nested SForms in args.
- **Matcher extension** for nested-fact patterns (Q29 compile pass
  treats SForm args as sub-patterns; runtime unifier recurses).
- **Engine emission**: P1.5's fork procedure emits `(hypothesis
  <fact>)`; the contradiction detector emits `(contradiction-under
  <fact>)`.

Pays F5 (rules-as-data) amortisation for free — fact-as-arg is the
precondition for rules-as-facts.

Promotes T1.3.2.5 (`hypothesis-contradiction`) back into the M1
core (was deferred under §F path (a) before Q40 resolution).

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q40.

## Q41 — Rule priority — scale and placement

Surfaced 2026-05-19 during the P1.3 review. Zebra.ein uses 1/3/5/8
priorities inside each rule definition. The original P1.3 plan
used 100/200/300/900 in a static config table.

**Resolved 2026-05-20** — adopt the **100/200/300/900 banded
scale** from the original P1.3 plan; keep zebra.ein's **per-rule
placement** (priority lives inside the `(rule …)` form, not in an
engine config table); **lower fires first**.

### Bands

| band       | priority | what fires here                                                  |
|------------|---------:|------------------------------------------------------------------|
| propagate  | 100      | cheap, structural propagation — symmetric, inverse, implies      |
| derive     | 200      | composition / join — transitive, square-fwd, square-bwd          |
| eliminate  | 300      | exclusion / narrowing — type-exclusivity                          |
| hypothesis | 900      | reserved for hypothesis-contradiction (P1.5)                      |

Within-band fine-grained ordering (e.g., 110 / 120) is open and
per-puzzle.

### Zebra.ein migration

`symmetric` 1→100, `implies` 3→100, `transitive` 5→200, `square-fwd`
8→200, `square-bwd` 8→200, `type-exclusivity` 10→300.

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q41.

## Q42 — P1.3 scope — path (a) vs (b)

Surfaced 2026-05-19 during the P1.3 review. Two paths:

- (a) Narrow P1.3 to the 6 zebra.ein rules + a seam for the other 4.
- (b) Keep all 10 with 4 marked `:blocked-by`.

**Resolved 2026-05-20 by Q33 cascade** — **path (a)**. Q33's
resolution (M1 ships only `eq` + `neq` predicates; all numeric /
set / variadic primitives → followups) makes the 4 deferred rules
structurally unshippable in M1. Path (b)'s `:blocked-by`
placeholders would be misleading — they'd point at followups, not
P1.4 / P1.7.

M1 core = 6 zebra.ein rules: `symmetric`, `transitive`, `implies`,
`square-fwd`, `square-bwd`, `type-exclusivity`. Plus
`hypothesis-contradiction` promoted in by Q40 = **7 rules total**.

The 4 deferred rules (T1.3.2.3 / .4 / .6 / .8 in the original
plan) land in a future followup phase adjacent to F4.

Full context:
`p1.3_inference_rules/s1.3.0_review_and_revisions.md` §G — Q42.

---

## Q-M1e.19 — Algorithmic pathology has no owner

**Raised 2026-08-29** by M1e S1e.1.6 (T1e.1.6.2), in M1e's ledger
(`docs/history/m1e_review_processing/open_questions.md`), as the one of the
review's `Q9` surfaces nobody swept: the parser and CLI edges were swept,
`cast.rs` went with `ein-einb`'s next change, micro-CSP ground truth is
[M10](m10_external_benchmarks/README.md)'s, and this one had no home. **Moved
here at M1f S1f.20.8 (2026-11-17)**, its id kept, when `docs/history/` was
dissolved: it is open, nothing in the project cites it, and every open
question lives in `plans/` (the user's ruling at S1f.20.2's review, item 10).
**Owner unassigned.**

*Where does the search degrade, and is there an input class that makes it
degrade catastrophically?* Nothing in the tree asks. What exists measures the
corpus as it is: `corpus/corpus.toml`'s `cost_ms` and `slow = true`
(`utils/corpus_cost.py`), and the budget's forecast, which tells a run stopped
at `--region-budget` what its next step would cost. **A pathology pass
measures what is *not* in the corpus**: a family of programs parameterised by
n, run until it stops finishing, with the exponent read off, the routes named
(the state route, the tree and the classes, route 1, the cube), and the
programs that put the alive set where nobody expects it found.

When it was raised the evidence was the deleted walk's: its tree traversal
reached `zebra2-minus-15-obligations.ein`'s 32 models in 86 enterings where
the lattice needed 17 204 592, a 200 053× gap on one input, and nothing said
which inputs have it. The walk went at M1e2 S1e2.1c.5; the question, which is
about the engine's inputs and not about one route, did not.

**Why it is not a task anywhere yet.** A pathology pass is a measurement
phase, a family of generated inputs, a harness, a fitted exponent and a
written conclusion, and no milestone has taken one. It is the natural
neighbour of [F22](followups/f22_exact_core_past_the_cap.md)'s unbounded core
search (K-82), which is one such pathology found by accident.

---

## The rulings about the process M1f left binding

**Moved here at M1f's close (2026-11-25)**, ids kept and each section as the milestone's ledger had it, when that
ledger went with the milestone's folder (`git log --diff-filter=D -- plans/m1f_hypothesis_and_documentation`). Each is
a ruling, or a default the user accepted or never revised, about **the process**, not about Ein: the gate and its
tiers, the nightly and the dev host, the CI image, the review bundle, the wrong-answer register and the scripts under
`utils/`. A decision about the process stays in `plans/` (the decisions under `docs/decisions/` are about Ein,
Q-M1f.346), and these still bind: `AGENTS.md`, `.github/workflows/`, `run_tests.sh`, `utils/` or a followup cites each
or states its rule. The milestone's other questions went to decision files under `docs/decisions/` where they answer a
question about Ein, and to git where they were spent; none was left open. Each stands until the user revises it. A
link in a section that pointed into the milestone's folder is its label text, or the decision file that now carries
the question it named.

### Q-M1f.13 — F3's cause reaches a disjunction: whose row is it?

**Raised** 2026-11-09 by S1f.5.1's T1f.5.1.3, step 1
(its record), which traced
P1f.5's F3: `_` in a rule's pattern reported root's core as four facts where a variable used once reported two. The
cause is a rule application's identity, its rule, activator and register file (`firing::BindingKey`): the saturator
drops a match whose key it has seen or fired, so a second match that binds every register as the first did is never
fired redundantly and never recorded as an alternative, whatever facts it matched. A premise's fact is a function of
the register file except through `Slot::Any`, which bound nothing, and across the disjuncts of one top-level `(or …)`,
which share their registers by name. With `_` refused since S1f.5.5's swap, the disjunction is left:
`examples/ein-bugs/wrong-answers/or-one-key.ein`, one rule `(or (p ?y) (q ?y))`, reports four facts where the same
rule written as two rules reports two. The brief: *a reach is a wrong answer, filed as a register row before anything
moves, and fixed if it is in scope*.

- **(a) File it `open`, deferred to F22**, as the user put W25 there on 2026-11-05 (Q-M1e2.16 (2) and (5): a
  derivation the saturator never offers to the store, from an activator a node derived). F22 decides which
  derivations a core is the smallest over, and how it gets them.
- **(b) Fix it in S1f.5.1**: the disjunct joins the application's identity (the `seen` and `fired` keys), so the
  second disjunct's match fires redundantly and records its alternative. It moves the cores of programs that write
  such a disjunction and the redundant firings `--events` narrates, and it changes the identity every fork inherits.
- **(c) Not a wrong answer**: the promise is *the smallest frontier over every derivation the program records*, and
  this derivation is never recorded. The register's W25 reads the promise over every derivation, recorded or not.

**Defaulted 2026-11-09 (the user's instruction): (a).** The user, 2026-11-09: *"just continue with default decisions
till the end of the phase"*. (a) is the agent's recommendation: it follows the user's ruling on W25, the same class of
defect, and keeps S1f.5.1, whose subject is `_`, from moving the saturator's dedup, which every match crosses. In the corpus at `1e751131` one program reaches the cause, `wild-in-head-twin.ein`, whose run
reports no core, so no printed answer moves today (the census, T1f.5.1.3's record). `W39/or-one-key` is the row.
Re-opening it means (b), in F22 or before it.

### Q-M1f.51 — Three more tests whose slow half no CI tier runs

**Raised** 2026-11-09 by S1f.5.13's
T1f.5.13.3, outside its brief. The stage's case was that `answer_round_trip`'s slow half ran in no CI tier, and the
same is true of others. Eight test targets read `EIN_CORPUS_SLOW` (`grep -rln EIN_CORPUS_SLOW ein.rs/crates`,
2026-11-09, less the three files that only name it): `corpus_cli`, `lever_invariance` and `corpus_cores` were nightly
steps before the stage, and `answer_round_trip` is one since. Of the other four, `read_out_examples`' slow branch adds
the page's blocks at `--region-budget 33554432`, and today's page has none. The other three have a slow half that only
`./run_tests.sh --slow`, run by hand, reaches:

| target | what `EIN_CORPUS_SLOW` adds | release, without · with the flag |
|---|---|---:|
| `ein-cli`'s `route_invariance` | the zebra family and the six `square-*`, at `--region-budget` 2²⁵ | 24.8 s · 857.3 s |
| `ein-infer`'s `region` | `the_state_route_is_route_1_on_zebra2_minus_15_through_layer_6` | 13.0 s · 37.6 s |
| `ein-infer`'s `obligation_rung` | `the_theory_finds_what_the_hrule_finds` on `zebra2-minus-15`'s 32 states | 3.4 s · 71.0 s |

Each passes at its slow depth (`build/s1f.5.13/t3_other_slow_halves.log`, 2026-11-09; run beside this stage's gate and
the second lane's tests, so the times are under load). No wrong answer is hiding there today, but if one of the three
started failing, nothing would notice.

- **(a) Recorded, not scheduled.** The stage carries P-16's ruling, which is about one test; the three stay under
  `--slow` until the user places them.
- **(b) A nightly step each**, beside the round trip's in `deep-corpus`, `route_invariance`'s the largest, and a test
  in `nightly_steps.rs` that fails on a target reading `EIN_CORPUS_SLOW` with no nightly step, so that the next one
  cannot slip in the same way.

**Defaulted 2026-11-09 (the user's instruction): (a)**, parked for the phase's close beside
[Q-M1f.8](../docs/decisions/engine/Q-M1f.8-the-lexer-is-lrlexs-unpatched.md): the stage stays in its scope, and nothing it
measured fails. Re-opening it means (b).

**Taken by S1f.5.27 on 2026-11-10**, when the user had accepted
(a) as parked and the close review's finding 8 re-opened the nightly: (b), by tier, under the stage's rule
([Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)). `region`'s and `obligation_rung`'s slow
halves are CI nightly steps, each a second route held to an answer on the `zebra2-minus-15` family that no per-commit
test reaches; `route_invariance`'s is a dev-host step, over 5 min, its programs' answers digested per commit by
`corpus_shapes`. The guard (b) asked for is `nightly_steps.rs`' `every_slow_half_runs_on_a_tier`: a target that reads
`EIN_CORPUS_SLOW` and that neither the nightly nor `--dev-host` runs with it fails, but `read_out_examples`, whose slow
branch reads only the page's blocks at `--region-budget 33554432`, and the page has none, which the test checks. Each
slow half's time is in the stage's records.

### Q-M1f.53 — How the per-commit gate catches a register row the bundle cannot read

**Raised** 2026-11-09 by S1f.5.19,
on the coordinator's brief: *make the per-commit gate catch this class of break*. The gate's `review_bundle.py
--smoke` builds `W2/forced-2` and one program, so its index read one row shape, a walk's verdict; the whole bundle,
the nightly's, is the only run that read W40's to W56's.

- **(a) The smoke builds a command case** beside its solve case, `W40/kb-save` or `W41/saturate-import`, so the index
  reads a refusal and a command's run. It catches today's shapes, and a shape the next stage files waits for the
  nightly again.
- **(b) (a), and every register row read with no run**: the index's register cells, each case page's § 1 table, its
  fixed callout and its checklist, written from `corpus/wrong_answers.toml` alone for all of its rows, each row named
  where one fails. A row shape the bundle cannot read fails the commit that files it. It runs no engine, so it costs
  milliseconds.

**Defaulted 2026-11-09 (the user's instruction): (b).** It is (a) and the class besides, at no measurable cost.
Re-opening it means dropping the register pass from `cmd_smoke`.

### Q-M1f.55 — A `container` on a solve's `now`

**Raised** 2026-11-09 by S1f.5.19.
`W43/einb-dump-answer`'s `now` is a solve with `container = true` (schema `/16`'s field, on a solve since `/19`):
`wrong_answers.rs` saves the witness with `ein kb save` and runs `ein solve` on the `.einb`. The case page ran it on
the `.ein`, and said *this build re-ran it*.

- **(a) Run the container**, as `wrong_answers.rs` does: the case page saves the witness first, shows that step, and
  every invocation of the run reads the `.einb`, the program page showing the witness's source as the program the
  container stores.
- **(b) Run the `.ein`**, and say so on the page.

**Defaulted 2026-11-09 (the user's instruction): (a).** The page then shows the run the register holds, and every
command the run drives takes a container since S1f.5.5's T13 and S1f.5.12's W43.

### Q-M1f.77 — When is `utils/ci_container.sh` part of done?

**Raised** 2026-11-10 by S1f.5.29. Twice a Graphviz refusal reached the per-commit CI behind
a green `./run_tests.sh` here: S1f.5.17's, the runner's x86-64-v3 `libpathplan` failing to route 20 views, and
S1f.5.29's, the runner's `fdp` with no layout plugin behind it, which refused every KB view the review bundle's smoke
draws, red on GitHub from `00c4ed34` on. In both, `utils/ci_container.sh`'s image, built like the runner, showed what
the runner showed. It runs HEAD as committed, so it is a check on a commit, not on a working tree.

- **(a) A stage that changes a DOT writer, the review bundle or what CI installs is not done until
  `utils/ci_container.sh` is green on its commit**, the gate's run in the image beside the local one, and the stage's
  commit or record says both. Cost: one more gate run per such stage, in the image, which needs an x86-64-v3 CPU and
  `docker`.
- **(b) Install the baseline Graphviz build in CI**, `runs-on` pinned or the amd64 variant forced, so the runner meets
  the build the local gate meets. It answers S1f.5.17's case only: S1f.5.29's plugin is missing from both 26.04
  builds, and the local Graphviz is neither.
- **(c) Run the smoke's drawings in the container from `run_tests.sh` itself**, so every local gate draws them on the
  runner's build. Every developer's gate then needs `docker`, a built image and the CPU, which the gate needs nowhere
  else.

**Defaulted 2026-11-10 (the stage's default): (a).** It is the stage file's default: it costs only the stages that
can break the drawing, and it would have caught both cases. Re-opening it means (b) or (c).

### Q-M1f.78 — Where the runner's missing `fdp` is fixed

**Raised** 2026-11-10 by S1f.5.29's T1. The runner's Graphviz 14.1.2 refuses every KB view the review bundle
draws, 612 of the whole bundle's 3 885 laid-out DOTs, with *There is no layout engine support for "fdp"*: every KB
view suggests `fdp` (`kb_dot::to_dot`), and Ubuntu 26.04's `graphviz` links `fdp` to `dot` without
`libgvplugin-neato-layout8`, the plugin behind it, which it neither depends on nor recommends. The stage file had
presumed S1f.5.17's cause and asked for a view-side fix; no DOT is at fault. Each fix below draws all 3 885 on the
runner's build (the stage's T1 record).

- **(a) The runner installs the plugin**: `libgvplugin-neato-layout8` beside `graphviz` on the apt line of
  `per-commit.yml`, `nightly.yml` and `release.yml` (its two, for parity), in `utils/ci_container.sh`'s image (only
  where the archive has the package: 24.04's `libgvc6` carries the plugin), and in the provisioning lines
  `gate_steps.rs` and `nightly_steps.rs` name; `run_tests.sh` exits 127 where `fdp` lays out no graph, as it does
  where there is no `dot`. No view, golden, answer or drawing moves, and the CI lays out what the user's bundle lays
  out. Cost: a 26.04 package name on the apt line, which a later image renames loudly (the install fails).
- **(b) The bundle falls back to `dot`** where the suggested engine does not load (one probe a run), and says so on
  the page. No golden moves, but on the runner the smoke checks a drawing the user's bundle does not make.
- **(c) The KB view stops suggesting `fdp`** (`kb_dot.rs`), so every KB view is laid out by `dot` everywhere. It moves
  the four DOT goldens that carry the line (`kb_render_to_dot.dot`, `slice_render_solution.dot`,
  `slice_render_state.dot`, `kb_zebra_unified.dot`), at least the 2 346 `dot[kb…]` lines of `corpus_shapes.md5`, the
  worked fragment in `04_dot_rendering.md`, and every KB drawing the user reviews, from force-directed to ranked.

**Defaulted 2026-11-10 (the coordinator's default): (a).** It fixes the cause, a package, and moves nothing. No user
ruling is reversed: the user's ruling on S1f.5.17 was not to report the Graphviz bug upstream, and *view-side* was
the stage file's presumption, not a ruling. Re-opening it means (b) or (c).

### Q-M1f.79 — Does a refused DOT fail the whole bundle?

**Raised** 2026-11-10 by S1f.5.29's T1. Since `00c4ed34` the smoke fails on a DOT with no SVG beside it, but
the whole bundle, the nightly's step, wrote a refused drawing on its page as *Graphviz refused this graph* and exited
0, a case failing only on an exception. So the GitHub nightly's bundle step drew no KB view from 2026-11-06, when the
runner moved to 26.04, and passed (the nightly was red then on later steps, S1f.5.17's mutants and S1f.5.19's row
shape), as S1f.5.17's in-container nightly at `76ede241` (*137 cases, 0 failed*) did in the same
image. Five DOTs in the whole bundle are kept undrawn on purpose: lattices past `DAG_NODES_DRAWN` (1 000 nodes),
whose page says so.

- **(a) Yes, as in the smoke**: `cmd_bundle` exits 1 on a DOT Graphviz refused, printing its path and what Graphviz
  said (kept beside the DOT as `.refused.txt`), over the cases it built. A DOT kept undrawn by design says so beside it
  (`.undrawn.txt`, `review.util.keep_undrawn`) and is not a refusal, told apart by what the bundle wrote, not by its
  size or path. The summary line counts the refused DOTs.
- **(b) No**: the nightly's bundle stays a review artefact, a refusal a line on its page, and only the smoke gates.
  A drawing the user reviews can then go missing with the nightly green, as every KB view did.

**Defaulted 2026-11-10 (the coordinator's default): (a).** A review artefact missing its drawings is what the user's
rule against incomplete review artefacts forbids, and the nightly is the only run that builds the whole bundle.
Re-opening it means (b).

### Q-M1f.102 — What the CI nightly keeps, and what the dev host runs

**Raised** 2026-11-10 by S1f.5.27, on the user's ruling on the close
review's finding 8: *"reduce nightly job, remove long running tests, we will eventually run them on dev host, retain
critical tests in CI"*. The ruling names two kinds of step and leaves their line to be drawn. The nightly took 81 min
54 s here on 2026-11-09, two fuzz seeds 51 min of it, and the latest run on GitHub 74 min 44 s, its fuzz jobs 75 and
59 min, one a seed beside the others (run 42802597417, 2026-11-10).

- **(a) The stage file's rule.** *Critical*: a step that guards an answer or a contract no per-commit step guards (the
  slow entries' answers, the config principle on the slow cells, the cores and dumps of the entries the per-commit tier
  leaves out, the suite's sensitivity). *Long-running*: over 5 min on the runner, or measuring rather than checking. A
  critical step stays in CI at a budget that fits and runs whole on the dev host; the rest run on the dev host only.
  `./run_tests.sh --dev-host` runs every nightly step at full budget and the steps that left, and nothing schedules it.
- **(b) A wall-clock budget for the nightly as a whole**, filled by priority. It needs a priority order the ruling does
  not give, and a step's place would move with the runner's speed, which varies up to 2× between runs (the stage's T1).
- **(c) Keep every step in CI, spread over more parallel jobs.** It keeps the long-running tests in CI, which the
  ruling removes.

**Defaulted 2026-11-10 (the stage's default): (a).** T1 re-took every step here and read three runs on the runner, and
confirmed the stage file's classification row by row, but the fuzzer's budget
([Q-M1f.103](#q-m1f103--the-fuzzers-budget-in-the-ci-nightly)): kept, the slow corpus entries' cells, `answer_dump`,
the round trip's, the levers' and the cores' slow halves, the mutation census and the bench smoke; added, `region`'s and
`obligation_rung`'s slow halves
([Q-M1f.51](#q-m1f51--three-more-tests-whose-slow-half-no-ci-tier-runs)); to the dev host, the id space permuted eight
ways, `stdlib_census.py --check`, the whole review bundle with its link check, and `route_invariance`'s slow half.
Re-opening it means (b), or a line other than 5 min.

### Q-M1f.103 — The fuzzer's budget in the CI nightly

**Raised** 2026-11-10 by S1f.5.27's T1. The stage file proposed 100 000
inputs a seed, *about 2.5 min*, from this machine's rate, a million in 1 532.7 s (S1f.5.19's T3 record). The runner's
rate is what the rule reads: a million took 4 410 s and 3 494 s (run 42802597417, 2026-11-10, six properties on four
vCPUs; seven since S1f.5.21), and 2 572 s and 3 020 s the day before (run 42605782048). Here, on four P-core threads,
the runner's count, 20 000 take 106.3 s and 100.1 s, and the properties that parse every input ran 1.13× and 1.34×
slower an input on the runner than here, so on the runner 20 000 come to about 2.7 and 3.3 min with the step's build
of about 50 s, and 100 000 to about 10 and 13 min.

- **(a) 20 000 a seed** in the nightly, ten times the per-commit tier's 2 000 and a fiftieth of the dev host's million.
- **(b) 100 000 a seed**, the stage file's number, past the rule's 5 min on the runner.
- **(c) Make an input cheaper**, so a larger budget fits: S1f.5.28's item 10 and
  [Q-M1f.8](../docs/decisions/engine/Q-M1f.8-the-lexer-is-lrlexs-unpatched.md), not this stage's.

**Defaulted 2026-11-10 (the stage's default): (a).** It is the rule's own budget, *a budget that fits*, read on the
runner's rate rather than this machine's. `nightly_steps.rs`' `the_fuzzer_runs_at_each_tiers_budget` holds it between
the per-commit 2 000 and the dev host's million. The first nightly after the push measures it on the runner.
Re-opening it means (b), or (c) first and a larger budget after.

### Q-M1f.105 — How the dev host's list is held

**Raised** 2026-11-10 by S1f.5.27's T2. The stage asks that a test pin the
dev-host list as `nightly_steps.rs` pins the nightly's, and that a step leaving the nightly appear in it.

- **(a) A third copy, held to the nightly's.** `run_tests.sh --dev-host` lists its steps under `dev_host_step "…"`
  banners; `nightly_steps.rs` holds every nightly step to be one of them, in order, at the dev host's fuzz budget, every
  other one to be named in `DEV_HOST_ONLY` with why, and the list to a floor of 17, so a step taken out of the nightly
  and left there fails until named, and one taken out of all three copies fails the floor. It pins the two fuzz
  budgets, which the diff reads past, the checked `TMPDIR`, and every target that reads `EIN_CORPUS_SLOW` to a step on
  one of the two lists (Q-M1f.51's (b)).
- **(b) One copy**, `--dev-host` calling `--nightly`'s steps with a budget variable: the nightly's steps are then not
  the workflow's text, which is what the diff compares, and the moved steps sit outside the shared block.
- **(c) A file of its own**, `dev_host_steps.rs`, with the workflow parse moved to `steps/mod.rs`: the same checks, and
  a restructure of `nightly_steps.rs` for no new property.

**Defaulted 2026-11-10 (the stage's default): (a).** Each step is read from the file that runs it, as `gate_steps.rs`
and `nightly_steps.rs` already do, and every way the copies can drift fails a test (the stage's T2 record checks each
by mutation). Re-opening it means (b) or (c).

### Q-M1f.106 — The whole review bundle on the runner's Graphviz

**Raised** 2026-11-10 by S1f.5.27's T1. Since
S1f.5.29 a DOT Graphviz refuses fails the
whole bundle, and the nightly's bundle step was the one CI run that drew all its DOTs on the runner's Graphviz with the
engine each suggests. The rule moves the bundle to the dev host: 372 s on the latest run, 740 s and 752 s on the two
before, and what it guards is the review's pages, not an answer.

- **(a) The dev host draws it**, here or in the runner's image (`utils/ci_container.sh --dev-host`). Per commit the
  runner still lays every view `ein-render` writes out with `dot` (`dot_wellformed`) and the smoke's 31 DOTs with the
  engine each suggests, `fdp`'s KB views and the trace blocks among them, and
  [Q-M1f.77](#q-m1f77--when-is-utilsci_containersh-part-of-done) is unchanged.
- **(b) Widen Q-M1f.77**: a stage that changes a DOT writer or the bundle also builds the whole bundle in the runner's
  image, `utils/ci_container.sh --dev-host` or the bundle alone in its `--shell`.
- **(c) Keep the bundle in the nightly**, past the rule's 5 min.

**Defaulted 2026-11-10 (the stage's default): (a).** The 612 refusals S1f.5.29 found were every `fdp` view, which the
smoke draws, and no view kind the bundle draws is outside `dot_wellformed`'s and the smoke's. Re-opening it means (b).

### Q-M1f.134 — What a register row records as `right` while F24's question is not ruled

**Raised** 2026-11-11 by S1f.7.9's T2. The stage files F24's three table rows `open`, owned by F24, and the right
answer of each depends on F24's question, (a) refused, (b) checked, (c) as given and marked, which is the user's
and is not ruled.

- **(a) One named reading, the others as alternatives**, the review `deferred`: how W8's rows record Q-M1e2.2 for
  F21 (`[case.right]` with a `reading`, `[[case.right.alternative]]` per other reading, class *contested*).
- **(b) No `right`**: the schema requires one.

**Defaulted 2026-11-11: (a).** `W87/written-rule-no-such-rule` states (b)'s refusal, which was S1f.7.9's own first
direction, with (a) and (c) as alternatives. The two core rows state (c)'s core, `(paid Ann)`, with (a)'s and (b)'s
refusals as alternatives: a row whose `right` is a refusal may not hold today's core beside it while a live row can
show it (`ein-corpus`'s `beside_a_refusal_now_records_what_today_reports`), and the core is what F24's table says
is wrong. The choice of reading rules nothing: F24's question stays the user's.

### Q-M1f.150 — The counter, and how the measured test is in the gate

**Raised** 2026-11-11 by S1f.6.3's T4, which asks for *a deterministic count of the matcher's work per request (a
counter of `ein_core::counters`, which the stage picks and names, not the wall clock)*. The counters are behind
the `counters` feature and compiled out of every build the gate made: an increment on the engine's hottest
path.

- **(a) `candidates`, in a gate step of its own:** `cargo test -p ein-infer --features counters --test
  arith_batch`, in `run_tests.sh` and `per-commit.yml`. `ein-infer` is built a second time with the feature,
  for one test; 15 s the first time here.
- **(b) The static check alone,** and the measured test run by hand: the check reads the module's text, and a
  cost that is not a second premise is not in the text.
- **(c) The feature on in the workspace's test build,** by a dev-dependency: every test binary, the `ein` the
  corpus sweep times among them, would count.

**Defaulted 2026-11-11: (a).** `candidates` is the facts the matcher offered to a premise and unified against,
which is what a scan inflates and nothing else does: 181.38 a request at 250, 1 000 and 4 000 requests. Without
the feature the same file holds the firings a request, in `cargo test --workspace`.

**Overruled 2026-11-12, and built** (S1f.6.21). The user,
on reply 46, option (b): *"(b) - we don't need this proportional gate"*. The step left `run_tests.sh` and
`per-commit.yml`; the test, the feature and the bounds stay, run by hand, in no tier.

### Q-M1f.284 — The worker pool's count, its heavy lane and its memory bound

**Raised** 2026-11-15 by S1f.6.39's T4. The stage file set the
count, `EIN_TEST_JOBS`, at 16 by default and at most the machine's threads, and left the lane and the bound to T1's
measurement: no corpus program's process went past 529 MB through the CLI or 272 MB in-process, and no pooled binary
past 1.80 GiB at sixteen workers (the stage's record).

- **(a) 16, a quarter of them for the heavy lane, the bound 24 GB**: the P-cores' sixteen threads' worth here, the
  runner's four by the cap; a program on `ein_corpus::pool::HEAVY` (T1's 256 MB or more: `send_more_money`,
  `features/04_open`) runs at most four at once.
- **(b) Every thread, 32**: the E-cores beside the P-cores, each E-core 1.7 times slower a thread (§ Today).
- **(c) The P-cores alone, by `taskset`**: no unit on an E-core.

**Defaulted 2026-11-15: (a), the user's to revise.**

### Q-M1f.285 — The nightly's and the dev host's test steps on the gate profile

**Raised** 2026-11-15 by S1f.6.39's T5. The nightly ran its test
steps `--release` (release codegen, no debug checks), the per-commit tier the dev profile (`opt-level = 1`, the checks
on); the gate profile is release codegen with the checks.

- **(a) Every test step of every tier on the gate profile**: the debug assertions and the overflow checks run on the
  slow cells too, which no tier checked them on; `corpus_cli`'s cost band reads release codegen as before; the
  mutation census runs the gate profile's `ein`, which the nightly's steps built.
- **(b) The per-commit tier on the gate profile, the nightly and the dev host `--release`**, as they were.

**Defaulted 2026-11-15: (a), the user's to revise.** `release.yml`, the release tier, is unchanged: it tests the
binary that ships.

### Q-M1f.287 — One pass for `corpus_shapes` and `id_order_invariance`

**Raised** 2026-11-15 by S1f.6.39's
T6, from E-72. `id_order_invariance`'s plain run of every (file, output) is the run
`corpus_shapes` digests: the same bytes, computed twice a commit, 30.6 s of `corpus_shapes` here (`b8c931f3`).

- **(a) Merge**: `id_order_invariance` digests its plain run against the golden, its floors and `EIN_BLESS` with it,
  and `corpus_shapes`' own pass goes; the re-bless command and the pages that name `corpus_shapes` follow. Leaves the
  two at about 65 s here, where they are 96 s.
- **(b) Keep both.**

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). Built by S1f.6.39's T7: `id_order_invariance`
digests its plain run against `corpus_shapes.md5`, its floors, the checkout's stdlib and `EIN_BLESS` with it, and
`corpus_shapes.rs` is gone; every check it made is made by the merged pass. Probe (T6, before the ruling): `id-leak`
moved 273 renderings of the golden and 483 pairs of the permuted sweep; a merged test asserts both.

### Q-M1f.288 — `jobs_invariance` over the programs that search

**Raised** 2026-11-15 by S1f.6.39's T6. A job count reaches only a region route's fan-out, which has more than one node
to fan out only in a program that searches: the 188 corpus programs whose describe saturates more than root
(`probes/sets.py`).

- **(a) The 188**, the list computed by the test from the program (a describe that saturates past root) rather than
  banked. `jobs-leak` moved 983 cells over them, of 987 over all (`t6.tsv.sets`). Leaves 7.5 s here, where it is
  20.2 s.
- **(b) Every program**, as now.

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). Built by S1f.6.39's T7, with Q-M1f.290 (a): per commit
the programs outside `tests/` that search, `corpus_ops::searches` (describe at the sweep's budget saturates more than
root), asked of each program; under `EIN_CORPUS_SLOW=1` every corpus program, the nightly's. Five other binaries catch
the same leak on fixed programs at four and eight jobs (`explore`, `region`, `register_region`, `region_modes`,
`line_modes_past_the_limit`).

### Q-M1f.289 — `id_order_invariance` over the programs that search

**Raised** 2026-11-15 by S1f.6.39's T6.

- **(a) The 188 programs that search**: `id-leak` moved 144 pairs over them (483 over all); the 33 frontend and shape
  outputs of the 629 others are no longer permuted, the load's shape and refusals still being `kb_semantics`' on every
  file. Leaves 14.6 s here, where it is 65.1 s.
- **(b) Every program, the region outputs over the 188 only.**
- **(c) Every program**, as now.

**Ruled 2026-11-15: (a), the full-corpus sweep moved to the nightly** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). Built by S1f.6.39's T7, with
Q-M1f.290 (a): per commit the permutation runs over the programs outside `tests/` that search, the golden over every
program outside `tests/`; the nightly runs both over every corpus program, in shards (Q-M1f.293).

### Q-M1f.290 — `tests/` out of the render and determinism sweeps

**Raised** 2026-11-15 by S1f.6.39's T6, § Today's candidate. The 220 programs under `tests/` are each written for one
stdlib rule's claim, and are 48 % of `corpus_shapes`' work and 52 % of `id_order_invariance`'s; their claims are
checked by `ein test` (`corpus_cli`'s and `test_cli`'s cells), the coverage census and the mutants.

- **(a) `corpus_shapes`, `id_order_invariance`, `jobs_invariance` and `dot_wellformed` over the 597 programs outside
  `tests/`**, the golden and `dot_wellformed`'s skip list re-banked for them. Every probe each caught over the corpus
  it still caught (`id-leak` 269 pairs, `jobs-leak` 942 cells, `neq-nested` and `states-maximal` renderings moved
  outside `tests/`); `neq-nested` is caught on `tests/` itself by `corpus_cli`, `test_cli` and `arith_search_free`.
  Leaves the four at 66.5 s here, where they are 129.5 s.
- **(b) With Q-M1f.288 and .289**: the determinism sweeps over the 179 programs that search outside `tests/`.
- **(c) As now.**

**Ruled 2026-11-15: (a), the same way** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): `tests/` leaves `corpus_shapes`' golden (now
`id_order_invariance`'s), the two determinism sweeps and `dot_wellformed` per commit; the nightly runs them over the
full corpus. Built by S1f.6.39's T7: the golden is one file, held per commit over the lines of the programs a run reads and
in the nightly over all of them, so nothing was re-banked; `dot_wellformed`'s skip list per commit is its entries
outside `tests/`. A corpus program with no golden line fails per commit.

### Q-M1f.291 — The five sweeps that set modes and routes against one another

**Raised** 2026-11-15 by S1f.6.39's T6, § Today's candidate: `explore`, `lever_invariance`, `route_invariance`,
`region_modes`, `claim_routes`. The probes: a state-route bug (`states-maximal`) was caught by `explore`,
`route_invariance`, `region_modes` and `claim_routes`, and by `region`'s, `register_region`'s and the golden's own
checks; a route-1 bug (`route1-maximal`) only by `region::route_1_is_the_oracle…` (within the oracle's limit) and by
`route_invariance`, on programs past the limit where nothing else holds route 1; `lever_invariance` caught none of the
six, its claim, that no config moves an answer, being held by no other sweep.

- **(a) Keep the five**; `route_invariance` per commit without the `slow` entries (the zebra family and
  `send_more_money`, 127.8 of its 165.0 s at one worker, `units-gate-j1.tsv`), which its slow half on the dev host
  runs at both budgets. Leaves it at about 8 s here, where it is 26.8 s; the zebra family's route equality leaves CI
  unless the nightly takes the slow half (Q-M1f.293).
- **(b) `route_invariance` over the programs past the oracle's limit only**, the rest being `region`'s two oracle
  sweeps' claim.
- **(c) As now.**

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): keep the five; `route_invariance`'s `slow` entries go to its slow half,
the nightly's. Built by S1f.6.39's T7 (where its 2²⁵ half runs: the record). No probe here argues for removing
`explore`, `region_modes` or `claim_routes`: each was one of four to catch the state-route bug, so the overlap is real,
but no single probe shows which one a fault would reach alone.

### Q-M1f.292 — One load a program, shared by the outputs that do not change it

**Raised** 2026-11-15 by S1f.6.39's T6, § Today's candidate. Of a `corpus_shapes` pass at the gate profile, 6.2 s is
`ir[resolve]` alone, reading and parsing the program's stdlib modules, and about 30 of the 41 outputs resolve again:
about 61 % of each of the three A sweeps' work.

- **(a) Load once a program and clone the loaded state for each output**: `Ast`, `Terms` and `Kb` would need
  `Clone`, which `Terms` and `Ast` do not derive — a change under `ein.rs/crates/*/src` beyond a test helper, so the
  user's to allow. Proven by invariance: `corpus_shapes`' golden reproduced byte for byte, and `id_order_invariance`'s
  permuted runs from the clone. Would leave the A sweeps at roughly half their 116 s here.
- **(b) Cache the resolved stdlib modules in the test helper** without a clone: not possible without the resolver's
  cooperation, which is the engine's.
- **(c) As now.**

**Ruled 2026-11-15: (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): one load a program, cloned for each output, `Clone` on `Ast` and `Terms` (and
`Kb`), an engine source change under which no output may move. Built by S1f.6.39's T7 (`corpus_ops::Loads`), and
verified by invariance: `corpus_shapes.md5` reproduced byte for byte from the copies, every other golden unchanged, and
the gate green (the run at one worker stopped after 99 binaries, every one `ok`, as redundant: T4 holds the count).

### Q-M1f.293 — The nightly's steps against five minutes

**Raised** 2026-11-15 by S1f.6.39's
T6, against [Q-M1f.102](#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)'s rule that a nightly step fits
in five minutes on the runner. On four P-core threads with four workers, the runner's count (`taskset -c 0-3 env
EIN_TEST_JOBS=4 ./run_tests.sh --nightly`, 2026-11-15): the mutation census 1 382.9 s, `corpus_cores`' slow entries
376.3 s, `answer_round_trip`'s slow and `heavy_dump` entries 353.0 s, and `region`'s slow half 271.7 s here, over five
minutes at the runner's 1.13 to 1.34 times (Q-M1f.103). The deep-corpus job is 47.7 min at that count.

- **(a) The four to the dev host**, each named in `DEV_HOST_ONLY` with why; the nightly keeps the slow cells, the
  levers', the imports' and `obligation_rung`'s slow halves, the fuzzer and the bench, about 8 min; the census and
  the four slow halves run where nothing schedules them.
- **(b) The census by shards**: a seventh of the mutants a night, the shard named by the weekday, `--check` on the
  shard's banked survivors, the week covering all 572; about 3.3 min a night. The three slow halves split by program
  the same way, or (a) for them.
- **(c) The rule relaxed for the census** (it guards the suite's sensitivity, which no per-commit step guards), the
  three slow halves to (a) or (b).
- **(d) As now**, past the rule.

**Ruled 2026-11-15: split into parallel nightly jobs (shards), staying in CI** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*). The nightly steps over five
minutes at the runner's four threads (the mutation census, `corpus_cores`' and `answer_round_trip`'s slow halves,
`region`'s slow half) and the full sweeps Q-M1f.289 and .290 add are sharded into parallel nightly jobs, each shard
at most five minutes measured as T6 did (`taskset -c 0-3`, `EIN_TEST_JOBS=4`, on AC). Built by S1f.6.39's T7:
`EIN_TEST_SHARD=k/n`, `ein_corpus::shard` and `stdlib_mutants.py --shard`; the shards and their times are the record's.

### Q-M1f.294 — `answer_dump`'s `heavy_dump` witness, and the feature tests the pool does not reach

**Raised** 2026-11-15 by S1f.6.39's T6. Two per-commit costs no sweep's program set explains:

- `answer_dump` dumps W8's witness `square-unique/corner-house.ein`, a `heavy_dump` entry, 33.2 s of its 33.8 s here;
  `answer_round_trip` left the `heavy_dump` entries to the nightly on the user's ruling of 2026-11-08 (P-16). **(a)**
  the same for `answer_dump` (leaves about 5 s); **(b)** as now.
- `int_theory`, `arith_batch`, `read_out_reference`, `read_out_examples` and `arith_theory`, feature tests, are
  65.5, 36.0, 23.3, 18.7 and 16.4 s here, 160 s of the gate's 887: **(a)** their loops on the pool (no check moves);
  **(b)** their long halves to the nightly; **(c)** as now.

**Ruled 2026-11-15: (a) and (a)** (the user, 2026-11-15: *"T6 rulings: all as you recommended"*): the `heavy_dump` witness to the nightly, and the five feature tests
on the pool where their loops allow it. Built by S1f.6.39's T7: `answer_dump` per commit leaves the `heavy_dump`
entries' runs, `EIN_CORPUS_SLOW=1` reads them; `int_theory`'s, `arith_theory`'s and `arith_batch`'s loops over
operations, `read_out_reference`'s example runs and `read_out_examples`' blocks are units of the pool. Neither is a
corpus sweep of § Today's 31.

### Q-M1f.296 — The bit tables, by the digit tables' generator

**Raised** 2026-11-15 by S1f.6.16's build (T1f.6.16.6, *"generator in `utils/`"*).

- **(a) `utils/gen_arith_tables.py` writes both** `stdlib/math/digits.ein` and `stdlib/math/bits.ein`, one
  `--check` over the two, held in the gate by `ein-ir/tests/digit_tables.rs`, which holds every row of both to Rust's
  integers; `digits.ein` byte for byte as before.
- **(b) A generator of its own**, `utils/gen_bits.py`, and a test of its own.

**Defaulted 2026-11-15: (a), the user's to revise.** The tables are one family at two bases, computed by one function
each in the script.

### Q-M1f.304 — `route_invariance` past the default budget, past any shard's five minutes

**Raised** 2026-11-15 by S1f.6.39's
T7, applying [Q-M1f.291](#q-m1f291--the-five-sweeps-that-set-modes-and-routes-against-one-another) (a) and
[.293](#q-m1f293--the-nightlys-steps-against-five-minutes): `route_invariance`'s `slow` entries go to its slow half,
the nightly's, each nightly step in shards of at most five minutes on the runner. Its slow half was two things: the
`slow` entries at the default budget, and every program route 1 finishes only past it (the zebra family, the six
`square-*` cubes) at `--region-budget` 2²⁵. Measured as T6 measured (`taskset -c 0-3`, `EIN_TEST_JOBS=4`, AC,
2026-11-15, `build/s1f.6.39/t7/slow-4.log`), the slow half ran 29 min unfinished, three workers busy throughout its
last 20, and was stopped there; and route 1 alone took 495 s at `-j 16` to reach a 2²² ceiling on `zebra2-minus-15`
(`alive_set_reach.md` § 2.3, 2026-11-03), where the
test asks 2²⁵ of one unit, one program. No sharding by program fits it.

- **(a) The default-budget half in the nightly, the 2²⁵ half on the dev host**: the test split in two,
  `the_state_route_and_route_1_print_one_read_out` (per commit the programs but the `slow` entries, under
  `EIN_CORPUS_SLOW=1` every one), which the nightly runs by name, and
  `route_1_and_the_state_route_agree_past_the_default_budget`, which `--dev-host` runs with the target whole
  (`DEV_HOST_ONLY`), as the whole slow half ran there since S1f.5.27.
- **(b) The 2²⁵ half in the nightly past the rule**, a step of its own over five minutes.
- **(c) A larger budget that fits**, between 2²⁰ and 2²⁵: the zebra family's route equality at a budget route 1
  does not exhaust is no comparison, so it holds of fewer programs.
- **(d) No test past the default budget.** Describe on a monotone program always takes the state route, and no
  option routes one to route 1, so the 2²⁵ half ran an internal route, `explore::describe_by`, at 32 times the
  default `--region-budget` of 2²⁰ the user ruled on 2026-11-03 after 2²⁴ ran the gate out of memory, on inputs no
  command sends it. The 2²⁵ test goes. In the default-budget test, a program the state route finishes and route 1
  does not is held by the cut, the state route's saturations and facts no more than route 1 spent before it
  stopped, and its read-outs are not compared; each is named on stderr. One neither route finishes is skipped, as
  before.

**Defaulted 2026-11-15: (a), the user's to revise.**

**Ruled 2026-11-15: (d).** The user: *"(d)"*, after the main session set out the default budget's ruling and what
the 2²⁵ half ran. Built at P1f.6's close: `route_1_and_the_state_route_agree_past_the_default_budget` deleted, and
with it the per-budget fold; `the_state_route_and_route_1_print_one_read_out` holds the cut's premise and names the
programs it holds by the cut and those it skips, and reads `EIN_CORPUS_SLOW` itself, so the nightly's step that
names it runs the target's whole slow half. Under `EIN_CORPUS_SLOW=1` (2026-11-15, the gate profile): 582 programs
compared; 5 held by the cut, `examples/zebra.ein`, `zebra2.ein`, `zebra2-obligations.ein` and the register's
`count-past-2-64.ein` and `count-past-2-128.ein`; 9 skipped, neither route finishing, the six `square-bwd` and
`square-fwd` programs, `send_more_money.ein` and `zebra2-minus-15.ein` with its `-obligations` twin. The dev
host's step and its `DEV_HOST_ONLY` entry are gone (`nightly_steps.rs`, `run_tests.sh`), and the pages that named
the half (`corpus/README.md`'s table, the workflow's comments).

### Q-M1f.314 — What the release-voice check reads, and what fails a line

**Raised** 2026-11-16 by S1f.20.1's
T1f.20.1.3. **Defaulted, to be shown at S1f.20.2's
review**, with the sweep's defaults. The stage file names the check's scope (`docs/ein/`, the shipped crates' doc
comments, `.ein` header comments, `corpus.toml`, `AGENTS.md` but § Where the work is, `README.md`) and what fails
(a milestone, phase, stage or task id, a path into `plans/`, a date that dates a development event; a decision id
passes). Seven readings it left open, each taken as below and built that way (`utils/check_release_voice.py`); the
baseline, 6 743 lines in 1 041 files, is of these readings.

1. **A shipped crate's doc comments** are every `///` and `//!` line of a `.rs` file under the six shipped crates,
   their `tests/`, `examples/` and `benches/` included: 3 271 baseline lines. *Alternative:* `src/` only. Plain `//`
   comments are not read; S1f.20.9 widens the check to code,
   comments and tests.
2. **A `.ein` file's comments** are all of them, not only the leading block: 2 501 baseline lines. One reader serves
   both checks (`doc_audit.py`'s, the text after a `;` outside a string), and a body comment speaks the same voice
   as the header. *Alternative:* the header block only.
3. **A date** is any ISO date, `YYYY-MM-DD`, in scope: no rule tells a development event from another date, and
   the escape, `release-voice-ok: <reason>`, covers the exception. *Alternative:* a date after *since*, *on*,
   *ruled*, *measured* and the like.
4. **A path into `plans/`** is `plans/` followed by a name, bare or relative; the folder named alone (*the stage
   records under `plans/`*) passes. *Alternative:* the folder's name fails too.
5. **What it does not read yet**: followup ids (`F21`), finding and register ids (`E-33`, `W93`, `K-82`), the M1e
   review's ids (`CD-H1`, `TE-L3`), and the strings of a `.ein` file (`:why`, `:source`, `:goal-text`), of which at
   least `stdlib/elim.ein`'s `:why "M1-blocking stdlib"` and the dates in `examples/lattice/synth/*.ein`'s carry
   the development. S1f.20.9 counts `E-n` and `D-n` as plan ids, so it widens the patterns with the scope.
6. **A count that falls is reported, not failed**: the check exits 0 and names the file to re-bank with `--bless`,
   as the stage file states the failure (*"where a count rises or a new file appears"*). *Alternative:* fail both
   ways, the baseline held exact as `tests/mutation_survivors.txt` is, so that a count lowered without a re-bank
   cannot later rise back to its banked number unnoticed.
7. **The check reads itself**, so its own docstring is held to the voice it enforces; its two lines of examples
   carry the escape, which were its first findings.

**Ruled 2026-11-17 as defaulted.** The user, at S1f.20.2's review, item 13, verbatim: *"Q-M1f.314: the release-voice
check reads every doc comment and every .ein comment, and every ISO date fails a line. A falling count is reported,
not failed. F/E/W/K ids are not read yet."*

**The readings S1f.20.9 changed**, when it widened the check and deleted its baseline: (1) every comment of every
crate, plain, trailing and block, the dev-only crates and the grammar's sources too, not a shipped crate's doc
comments; (4) `plans/` named alone fails, as `utils/` does: the project names no process; (5) followup, finding,
register and review ids are read now, and what is still not read is a `.ein` string (`:why`, `:source`,
`:goal-text`), a Rust string, a plan's `D<n>`, a `C<n>` case and a commit hash (L-2, L-49, L-54 and L-85 of
the milestone's findings page); (6) moot: with no baseline nothing is banked, so nothing falls; (7)
the check does not read itself. The scope also widened from `docs/ein/`, `docs/decisions/` and `README.md` to
every `.md` of the project, and gained `corpus/regressions.toml`; `AGENTS.md` left it, being the process's.
Readings (2) and (3) stand. P1f.20's folder, with both stages' records, is deleted
(`git log --diff-filter=D -- 'plans/m1f_hypothesis_and_documentation/p1f.20_*'`).

### Q-M1f.496 — The `fdp` provisioning, now that no view suggests `fdp`

**Raised** 2026-11-18 by S1f.23.6's build, reading 10. **Put, not ruled.** The layered layout dropped the KB view's
`// suggested layout: fdp`, so the layout plugin guards no drawing the gate makes; `render_examples.sh` alone still
picks `fdp`, for files named `*ontology*` or `*constraints*`.

- **(a)** left as it is: the CI's `libgvplugin-neato-layout8`, `run_tests.sh`'s exit 127 where `fdp` lays out
  nothing, and the comments in `AGENTS.md`, the two workflows, `run_tests.sh`, `utils/ci_container.sh`,
  `utils/review_bundle.py` and `utils/review/util.py` that say every KB view suggests `fdp`, which are wrong from the
  stage on. **Default.**
- **(b)** the comments brought to the layered layout, and what CI installs and `run_tests.sh`'s check taken again.

**Default taken: (a).** Nothing built: what CI installs, and the comments, the stage left to the main session.

**Ruled 2026-11-19: the default**, by the user's *"accept all remain defaults in this phase"* (P1f.23).

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
