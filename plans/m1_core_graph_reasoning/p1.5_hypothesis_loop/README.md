# P1.5 — Hypothesis loop + ATMS-style branching

**Estimate:** ~14-18 days (revised by S1.5.0 + the S1.5.4
hyp-gen-improvements stage added 2026-05-21 after the demo-run
feedback).
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
| S1.5.4  | [Hypothesis generation improvements — relation closure + dead-hypothesis cache](s1.5.4_hypgen_improvements.md) | 3-4 days |

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
  ([S1.5.4 T1.5.4.5](s1.5.4_hypgen_improvements.md#task-t1545--acceptance)).
- Unconditionally-dead hypotheses are cached for the duration of
  a single `solve()` call (S1.5.4 T1.5.4.3) — the same `(R A B)`
  is never re-tested at a deeper depth.
- `pytest tests/inference/test_hypothesis.py
  tests/inference/test_multilevel.py
  tests/inference/test_canonicalisation.py
  tests/inference/test_hypgen_pruning.py` ≥ 36 tests, green.
- `ruff check src/ein_bot/inference/hypothesis.py
  tests/inference/test_*.py` green.

## Connections

- [Idea 05 §Hypothesis mechanism](../../../docs/ideas/05-zebra-puzzle-graph-reasoner.md#hypothesis-mechanism)
  — the PoC's sketch, done right.
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
  [docs/index/09-cognitive-architectures-neurosymbolic.md](../../../docs/index/09-cognitive-architectures-neurosymbolic.md).
- [P1.4 contradiction detector](../p1.4_constraints/s1.4.1_contradiction_detector.md)
  — the dead-leaf test for each branch.
- [P1.8 Theme B2 (COW fork)](../p1.8_ein_lang_modules/README.md)
  — follow-up that makes `kb.fork()` O(1); compounds with dedup
  to drive M1's solve-time down. Out of M1 scope.
- [F1 categorical formulation](../../followups/f1_categorical_formulation.md)
  — the proof DAG is the morphism object F1 would formalise.
