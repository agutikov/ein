# P1.5 — Hypothesis loop + ATMS-style branching

**Estimate:** ~14-18 days for the core M1-blocking stages
(S1.5.0–S1.5.4 + the S1.5.4a/b prerequisite cleanups), plus
~7-10 days for S1.5.7 + S1.5.8 — also M1-blocking, for the
idea-08 trace-fidelity criterion (see the stage note below).
S1.5.6 is an optional pruning follow-up (2-3 days when
activated) and does not gate M1 acceptance; S1.5.5 was deferred
whole to P1.8 on 2026-05-22.
**Depends on:** P1.2 (`KnowledgeBase`, `kb.fork`, `Provenance`),
P1.3 (Saturator, hypothesis-contradiction rule), P1.4
(`ContradictionDetector`).
**Blocks:** P1.7 (Zebra integration).

## Goal

Implement the **outer loop** of the reasoner — the part that
takes over when P1.3's saturator stalls without solving. The loop
branches on hypotheses (one per under-bound `(relation, slot,
filler)` triple for the most-constrained object), tests each
branch by re-saturating, and either accepts a unique survivor or
recurses.

Re-framed by [S1.5.0 review](s1.5.0_review.md) §F: **the search
tree the loop builds IS the proof**. Dead branches are not noise
— their unsat-cores are the constructive `¬H` sub-proofs that
witness the surviving branch's uniqueness. P1.6's trace renderer
serialises the whole tree; idea-03's three task classes (solve /
gaps / contradictions) all read the same artefact.

Per [`docs/ideas/03-three-task-classes.md`](../../../docs/ideas/03-three-task-classes.md),
the same loop services all three task classes; the difference is
what the loop *records* and what it returns at quiescence.

## Stages

| ID      | Title                                                                        | Duration |
|---------|------------------------------------------------------------------------------|----------|
| S1.5.0  | [Review against shipped P1.2–P1.4](s1.5.0_review.md)                          | meta     |
| S1.5.1  | [Solve driver + two-step hypothesis gen + Q40 wiring](s1.5.1_saturate_branch.md) | 5-6 days |
| S1.5.2  | [Multilevel branching + search tree as proof object](s1.5.2_multilevel.md)    | 3-4 days |
| S1.5.3  | [Canonical fact-list hash + state-dedup + alive-branch termination](s1.5.3_canonicalisation.md) | 3-4 days |
| S1.5.4a | [Fix `single-parent` → `functional` (+ optional direct ⊥-rule kernel)](s1.5.4a-fix-single-parent.md) | ½-2 days (prerequisite) |
| S1.5.4b | [Drop Filter B (slot-already-used) from `_hypotheses_for`](s1.5.4b-fix-filter-slot-already-used.md) | ½ day (prerequisite) |
| S1.5.4  | [Hypothesis-gen improvements — `(closed R)` + config head + counters + alive-set polish](s1.5.4_hypgen_improvements.md) | 3-4 days |
| S1.5.5  | [Closure auto-inference](s1.5.5_closure_auto_inference.md) — **deferred to P1.8** (2026-05-22) | — |
| S1.5.6  | [One-step rule lookahead + `sibling-exclusive` 2-arg rewrite](s1.5.6_one_step_lookahead.md) | 2-3 days |
| S1.5.6b | [Guided hypothesis generation](s1.5.6b_guided_hypgen.md) — done (2026-05-22) | ~3-5 days |
| S1.5.7  | [Back-prop `(not h)`, re-saturate, return on derived positive](s1.5.7_back_prop_unconditional.md) | 4-6 days |
| S1.5.7b | [Stable-alive caching in the `_consume` loop](s1.5.7b_consume_loop_stable_alive.md) — parked (2026-05-23), not M1-blocking | ~1-1½ days |
| S1.5.8a | [Relation projections + property-on-projection design](s1.5.8a_relation_projections_design.md) — design alternative A (StructuralScan); **superseded by S1.5.8c** | meta |
| S1.5.8b | [Minimal-kernel proposal for domain-elimination](s1.5.8b_minimal_kernel_proposal.md) — design alternative B (precursor); **rationale record, refined by S1.5.8c** | meta |
| S1.5.8c | [Final decision](s1.5.8c_final_decision.md) — locks the design: 3 kernel deltas + 2 parser sugars + 2 grammar chars + 10-rule ein stdlib (6 M1-blocking + 4 optional closure) + B1 zebra2 refactor. Tasks T1.5.8c.1–T1.5.8c.6 shipped (commits `04f5d56`, `615b22c`, `0d7f348`, `f63ce9b`, `2b7f982`). **T1.5.8c.7 (acceptance) spun out to [P1.5a](../p1.5a_zebra_solution/README.md)** — getting zebra to actually solve hit a saturator NAF-semantics race and a hypothesis-count perf gap that need their own design pass. | meta |
| S1.5.8  | [Totality + domain elimination](s1.5.8_totality_domain_elimination.md) — superseded by S1.5.8c; implementation shipped across commits `04f5d56`, `615b22c`, `0d7f348`, `f63ce9b`, `2b7f982` | meta |
| S1.5.9  | [ein-lang pattern macros](../p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md) — **moved to P1.8 Theme A** (2026-05-24); the stage id stays S1.5.9 for stable cross-refs; the work is stdlib-shaped (macros replace compile.py SForm desugaring) so it lives with imports + stdlib in P1.8 | ~2-3 days |

S1.5.5/6/7 split out of S1.5.4 on 2026-05-21 per the
implementation-order TODO; S1.5.8 was added 2026-05-22. Each
ships behind a config flag from T1.5.4.4's `(config …)` head (or,
for S1.5.8, as a rule loaded from the puzzle); each is
independently testable.

- **S1.5.5** — **deferred whole to P1.8** (2026-05-22): closure
  auto-inference is a stdlib-rule concern, re-homed to P1.8's
  modules + imports + standard library theme. The manual
  `(closed R)` activator (S1.5.4 T1.5.4.1) stays and is shipped;
  only the *automatic* inference of `(closed R)` deferred.
- **S1.5.6** — pure pruning optimisation; does **not** gate M1
  acceptance. S1.5.4's acceptance (T1.5.4.6) closes the core
  hypothesis-loop phase.
- **S1.5.6b** — **done** 2026-05-22; hypgen-steering — relation
  whitelist, `hrule` rule-driven generation, the
  `enable-auto-hypgen` on/off flag. The proactive counterpart to
  S1.5.6's reactive filtering; not M1-blocking.
- **S1.5.7b** — **parked** 2026-05-23; perf follow-up to S1.5.7.
  Today's consume loop re-runs `try_branch` on every still-alive
  candidate every sweep pass — under M1's rule set re-saturation
  can't change an alive verdict, so the re-tries are pure waste
  (demo 10 measured 110 `try_branch` calls flag-on vs 70 flag-off
  for the same 32-vs-50 tree). Tracks a per-candidate
  "verified-alive after re-saturation generation N" cache; not
  M1-blocking.
- **S1.5.7 / S1.5.8** — **M1-blocking** (2026-05-22 direction "M1
  has to solve zebra"). S1.5.7's re-saturation +
  return-on-derived-positive and S1.5.8's `domain-elimination`
  rule together make a forced move ("therefore X") a named
  saturation firing instead of a hypothesis-branch verdict —
  required by M1 acceptance criterion #3 (the idea-08 trace must
  show a named `elimination-by-exhaustion` firing for every
  "therefore" move in the human walkthrough). They also remove
  the depth-budget exhaustion the static descent risked when
  forced moves each spent a tree level.
- **S1.5.8a / S1.5.8b / S1.5.8c** — **design meta-stages**
  added 2026-05-23/24. All adopt B1 (drop sub-domains;
  refactor zebra2 into specific binary relations).
  - **S1.5.8a** — initial design with Python-side
    `StructuralScan` opcode. **Superseded** by S1.5.8c.
    Specific zebra2-refactor decisions reaffirmed.
  - **S1.5.8b** — minimal-kernel pivot (drop NAF default;
    add `absent`; add `forall` opcode). **Refined** by
    S1.5.8c (forall becomes parser sugar, `open` added).
    Kept as the rationale record.
  - **S1.5.8c** — **final decision** (2026-05-24). 3 kernel
    deltas (drop `(not P)` NAF default; rename
    `NegativeGuard` → `AbsentGuard` / register `absent`
    head; `*` in SYMBOL/VAR grammar) + 2 parser sugars
    (`forall` → nested absent; `open` → conjunction of
    absents) + the 10-rule ein stdlib block (6 M1-blocking
    + 4 optional closure facility) + B1 zebra2 refactor.
    ~20 LOC kernel + 20 LOC parser pass + 2 grammar chars.
    S1.5.8 implementation reads only this file.

The original `s1.5.3_symmetry.md` (engine-time symmetry breaking)
was **dropped** during the S1.5.0 review — symmetry is an
ontology-writer concern, not an engine one. See
[S1.5.0 §D](s1.5.0_review.md#d-symmetry--not-an-engine-concern-s153-dropped)
and the updated
[Q6 resolution](../open_questions.md#q6--symmetry-breaking).

## Acceptance

- `solve(kb, mode=…, max_depth=…)` returns
  `Solution | Ambiguity | Contradiction` (per
  [S1.5.1 T1.5.1.1](s1.5.1_saturate_branch.md#task-t1511--solve-driver)).
- The Zebra puzzle solves uniquely under `Mode.SOLVE`:
  `(co-located Norwegian Water)` and
  `(co-located Japanese Zebra)` are in the resulting KB.
- Zebra-minus-condition-15 produces an `Ambiguity` with at least
  two `Solution` branches.
- Zebra-plus-a-conflicting-positive produces a `Contradiction`
  whose `unsat_core` traces back to the contradicting fact + at
  least one Zebra source condition.
- The returned `Solution` carries a `SearchTree` (the proof
  object); `tree.dead_branches()` is non-empty for non-trivial
  puzzles (uniqueness needs dead branches as witnesses).
- Search-tree round-trips through `(trace …)` IR — `to_ir()` then
  `parse()` + `from_ir()` reconstructs the same tree.
- The `(symmetric R)` activator causes the hypothesis generator
  to emit BOTH orderings per pair, not one canonical ordering
  (S1.5.1 T1.5.1.2d).
- Dedup via `state_hash` collapses sibling branches that reach
  the same post-saturation KB (S1.5.3 T1.5.3.2).
- The `solve()` driver does NOT return early on the first
  goal-matching branch — alive-branch termination is enforced
  (S1.5.3 T1.5.3.4).
- `(closed R)` declarations cause the hypothesis generator to
  skip relation `R` entirely (S1.5.4 T1.5.4.1); the demo-suite
  node counts collapse to the human-reasonable shape
  ([S1.5.4 T1.5.4.6](s1.5.4_hypgen_improvements.md#task-t1546--acceptance)).
- Hypothesis generation is observable: raw + per-filter
  counters via `bench_solve --hyp-stats` (S1.5.4 T1.5.4.7) and
  on the SearchTree's root metadata.
- The alive hypothesis set is computed once at b0 and inherited
  by descendant branches (S1.5.4 T1.5.4.5 / T1.5.4.8); revert
  via `(config :enable-alive-inherit false)`.
- A `(config …)` IR head + `solve(kb, *, config=…)` kwarg gate
  every optional pruning mechanism (S1.5.4 T1.5.4.4); enabling
  the S1.5.7 default-off `:enable-back-prop-unconditional` flag
  caches unconditionally-dead hypotheses for the duration of a
  single `solve()` call — the same `(R A B)` is never re-tested
  at a deeper depth.
- With S1.5.7 + S1.5.8 enabled, a one-survivor slot is resolved
  by a named `elimination-by-exhaustion` firing recorded on the
  current node — not a depth+1 hypothesis branch; the Zebra
  search tree's interior nodes are genuine choice points only,
  and the trace reads like the idea-08 human walkthrough
  (S1.5.8 T1.5.8.5).
- `pytest tests/inference/test_hypothesis.py
  tests/inference/test_multilevel.py
  tests/inference/test_canonicalisation.py
  tests/inference/test_hypgen_pruning.py
  tests/inference/test_config.py` ≥ 36 tests, green.
- `ruff check src/ein/inference/hypothesis.py
  tests/inference/test_*.py` green.

## Connections

- [Idea 05 §Hypothesis mechanism](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#hypothesis-mechanism)
  — the 2021 prototype's sketch, done right.
- [Idea 06 row 5](../../../docs/ideas/06-inference-rules-completeness.md)
  — hypothesis-and-contradiction as a *rule*; M1 ships it as
  P1.3's `hypothesis-contradiction` rule, P1.5 supplies the
  protocol that triggers it (Q40 Option A).
- [Idea 03 §The implicit fourth class](../../../docs/ideas/03-three-task-classes.md)
  — *explanation* falls out of the SearchTree-as-proof artefact.
- [Idea 08 — human-style trace](../../../docs/ideas/08-human-style-deductive-trace.md)
  — the walkthrough corresponds to a traversal of the SearchTree
  (including dead branches).
- ATMS prior art:
  [docs/lib/09-cognitive-architectures-neurosymbolic.md](../../../docs/lib/09-cognitive-architectures-neurosymbolic.md).
- [P1.4 contradiction detector](../p1.4_constraints/s1.4.1_contradiction_detector.md)
  — the dead-leaf test for each branch.
- [P1.8 Theme B2 (COW fork)](../p1.8_ein_lang_modules/README.md)
  — follow-up that makes `kb.fork()` O(1); compounds with dedup
  to drive M1's solve-time down. Out of M1 scope.
- [F1 categorical formulation](../../followups/f1_categorical_formulation.md)
  — the proof DAG is the morphism object F1 would formalise.
