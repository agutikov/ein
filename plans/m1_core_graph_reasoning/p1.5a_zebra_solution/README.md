# P1.5a — Zebra solution + saturator/NAF gaps

**Estimate:** unknown — depends on which path is chosen (engine
re-arch vs ein-side workarounds vs hybrid).
**Status:** **active** parking lot — created 2026-05-24 as the
follow-up phase that owns the work originally framed as
[S1.5.8c.7](../p1.5_hypothesis_loop/s1.5.8c_final_decision.md#task-t158c7--acceptance)
acceptance. Spun out because the underlying gaps are deeper than
"land a few more rules" — they're engine semantics + rule-set
completeness questions that need their own design + measurement.
**Depends on:** [P1.5](../p1.5_hypothesis_loop/) shipped through
[S1.5.8c](../p1.5_hypothesis_loop/s1.5.8c_final_decision.md);
zebra2 in B1 encoding with bijective shorthand + disjunctive-prune
landed (commits `04f5d56`, `615b22c`, `0d7f348`, `f63ce9b`,
`2b7f982`).
**Blocks:** M1 acceptance criterion #2/#3 from
[M1 README](../README.md#acceptance) — "zebra solves uniquely
under Mode.SOLVE" and "the trace matches idea-08 to within named
rule firings."

## Goal

zebra2.ein, in its current B1 encoding with the rule set shipped
through S1.5.8c, **doesn't actually solve in reasonable time** —
bench_solve runs to Ambiguity with one open branch even at depth
0 (no hypothesis branching). The structural refactor is complete;
the inference gap that prevents solve is the work of this phase.

## What's wrong, concretely

### (a) NAF-at-enqueue-time race

The saturator's `AbsentGuard` evaluates its sub-plan against the
KB state **at the moment of first enqueue**, not at fire time.
Once a binding-with-NAF-passes lands in the queue, the firing
commits regardless of later KB state.

This races with rules whose NAF premises depend on facts that
are themselves *derived* by other rules (e.g.,
`cross-attr-spatial-fwd`'s `(absent (?S ?h_o ?h1))` against a
`next-to` relation populated by `(symmetric)` + `(includes)`).
Between the first derivation step and the last, the NAF sees a
partial KB and passes spuriously.

Surfaced concretely:
- `cross-attr-spatial-fwd` for condition (12) `(Kools next-to
  Horse)` fires deterministically for non-corner Kools houses,
  inventing a unique-neighbour conclusion when the puzzle's
  semantics says the case is disjunctive.
- The `disjunctive-prune` rule (intended to derive negatives
  for non-adjacent houses) has the same exposure since it has
  the same NAF shape.

Workaround in current zebra2: declare next-to facts explicitly
in `(facts)` (8 facts replacing 4 `right-of` + `symmetric`/`includes`
derivation). Dodges the race for *this* relation; doesn't fix the
underlying semantic.

### (b) Hypothesis-count perf gap

Root-hypothesis enumeration produces 122 candidates under the
current B1 encoding vs ~101 under the pre-B1 zebra2. The 21-candidate
delta tracks to losing the `(sibling-exclusive is-a co-located)`
firings — pre-B1 those derived ~120 `(not (co-located A B))` facts
for same-type sibling pairs, which the hypgen filter consumed to
prune candidates.

Under B1 there's no `co-located`; `(sibling-exclusive is-a
house-color)` etc. would assert malformed negatives
(`(not (house-color Red Blue))` — args don't satisfy house-color's
House×Color signature). So no comparable pruning exists.

### (c) idea-08 trace fidelity

M1 acceptance #3 requires every "Therefore X" in the
[idea-08 target trace](../../../docs/ideas/08-human-style-deductive-trace.md#the-target-trace-paraphrased)
to correspond to a named saturation-rule firing in the engine
trace. Until the puzzle actually solves, this criterion can't
be checked. After solve works, the trace's named-rule firings
need to be diffed against the human walkthrough.

## Stages

| ID         | Title                                                                                    | File                                                                |
|------------|------------------------------------------------------------------------------------------|---------------------------------------------------------------------|
| S1.5a.1    | NAF semantic re-architecture                                                              | [s1.5a.1_naf_semantic_rearch.md](s1.5a.1_naf_semantic_rearch.md)    |
| S1.5a.1a   | Branch exploration order determinism                                                      | [s1.5a.1a_branch_order_determinism.md](s1.5a.1a_branch_order_determinism.md) |
| S1.5a.2    | Hypgen pre-pruning recovery                                                               | [s1.5a.2_hypgen_pre_pruning_recovery.md](s1.5a.2_hypgen_pre_pruning_recovery.md) |
| S1.5a.3    | idea-08 trace acceptance                                                                  | [s1.5a.3_idea08_trace_acceptance.md](s1.5a.3_idea08_trace_acceptance.md) |
| S1.5a.4    | Acceptance — zebra2 solves uniquely                                                       | [s1.5a.4_acceptance_zebra2_solves.md](s1.5a.4_acceptance_zebra2_solves.md) |
| S1.5a.5    | zebra2 relation rename family (`*-loc`, `co-located`, `adjacent-via`)                     | [s1.5a.5_house_to_location_rename.md](s1.5a.5_house_to_location_rename.md) |
| S1.5a.6    | PyPy compatibility + perf measurement                                                     | [s1.5a.6_pypy_compat_perf.md](s1.5a.6_pypy_compat_perf.md)         |
| S1.5a.7    | Hypothesis scoring + branch-info ordering                                                 | [s1.5a.7_hypgen_scoring_branch_info.md](s1.5a.7_hypgen_scoring_branch_info.md) |
| S1.5a.8    | Static NAF dependency map (observability)                                                 | [s1.5a.8_naf_dependency_map.md](s1.5a.8_naf_dependency_map.md)     |
| S1.5a.10   | Query semantics: who vs where                                                             | [s1.5a.10_query_semantics_who_vs_where.md](s1.5a.10_query_semantics_who_vs_where.md) |

S1.5a.1 + S1.5a.2 + S1.5a.3 close M1 acceptance criteria #2/#3
via S1.5a.4. S1.5a.5–.10 are non-blocking polish + perf +
observability + query-semantics investigations; promote
individually if their gating signals arrive. S1.5a.5 already
shipped 2026-05-24 (the rename ended up broader than initially
scoped — it folded in the `linked` → `co-located` and
`cross-attr-spatial` → `adjacent-via` renames discovered as
TODOs in the same file). S1.5a.8 (NAF dependency map) was spun
out of S1.5a.1 2026-05-24 once T1.5a.1.1's runtime fix shipped
— the static-warning half is pure observability with its own
acceptance bar. S1.5a.1a (branch order determinism) inserted
2026-05-24 — discovered while measuring [S1.5a.2](s1.5a.2_hypgen_pre_pruning_recovery.md)
candidates; `kb.alive`'s `frozenset` plus randomised
`PYTHONHASHSEED` made every `bench_solve` invocation visit
branches in a different order. A prerequisite for any A/B
measurement and for golden-tree snapshots downstream. S1.5a.10
(query semantics) surfaced during the rename pass from a
line-514 TODO; the natural human question "who drinks water?"
returns a House under the current goal, not a Nationality —
goal-extension is the cheap fix, `:project` is the longer-term
refactor.

## Relation-name refactor: `house-*` → `*-loc` (shipped)

Shipped 2026-05-24 — broader than originally scoped. The five
`house-*` relations renamed to `*-loc` (color-loc, nation-loc,
drink-loc, smoke-loc, pet-loc) with attribute as slot 0 and
House as slot 1; `linked` renamed to `co-located` and
`cross-attr-spatial` renamed to `adjacent-via` (with `?S` moved
to the first arg). All folded into [S1.5a.5](s1.5a.5_house_to_location_rename.md);
that doc has the full diff. The relations read as "location of
X" — `(color-loc Red H1)` ≡ "the location of Red is H1" —
matching the way `(co-located nation-loc Englishman color-loc Red)`
already wants to be read. Saturation behaviour unchanged
(223 firings, 181 productive, identical to pre-rename).

## PyPy

Surfaced 2026-05-24. The solver is pure Python with no native
extensions; PyPy is a natural candidate for a free speedup,
especially for the hot saturator loop (`_apply` + match
binding). Two stages of work:

1. **Compatibility audit** — run the existing pytest suite under
   PyPy. Likely-incompatible bits: Lark grammar parsing
   (Lark *does* support PyPy in recent versions), any C-extension
   dependencies (none today, but check `pyproject.toml`), any
   `@dataclass` corners that interact with PyPy's slot model.
2. **Perf measurement** — `bench_solve examples/zebra2.ein` under
   CPython vs PyPy, plus the branching demos. Headline metric: solve
   time on zebra2 once the NAF + hypgen fixes (S1.5a.1 / .2) land.

If PyPy gives a free ≥5× on the hot loop, it changes the
calculus on Theme B1 (indexes) in P1.8 — the saturator might not
need the index work as urgently. Either way, document the
finding so future perf work knows the baseline.

Out of scope here: maintaining PyPy-compatibility as a
permanent constraint (i.e. blocking CPython-only constructs). The
goal is "measure and document", not "support forever".

## Hypothesis scoring and branch-info ordering

Surfaced 2026-05-24 from TODO.md's "P1.5a ideas" entry. Two
related directions for better hypothesis-loop ergonomics, plus a
guiding question.

### Ideas

**1. Hypothesis scoring by fact-popularity sum.** For a candidate
hypothesis with the shape `(?R ?A ?B)` (one relation + two
objects, the M1 hypothesis shape), score by a weighted sum of
the components' existing-fact counts:

```text
score(R)  = number of facts mentioning relation R
score(A)  = number of facts mentioning object A
score(B)  = number of facts mentioning object B

1st-level score = rel_weight * score(R) + obj_weight * (score(A) + score(B))
```

The multiplier weights `rel_weight` and `obj_weight` are
configurable knobs; tune them empirically. A second round
extends the score by summing facts at *one hop* — the objects
that the candidate's components co-occur with in existing facts.

Worked example (from the original TODO note):

```text
(co-located Red House1)    ;; fact 1
(co-located House1 Milk)   ;; fact 2

score(House1) = 2, score(co-located) = 2, score(Red) = 1
score(Milk) = 1, score(Cat) = 0

hypothesis (co-located House1 Cat):
  1st-level = score(co-located) + score(House1) + score(Cat) = 2 + 2 + 0
  2nd-level adds score(Red) + score(Milk) (the 1-hop neighbours)
```

The intuition: hypotheses involving heavily-referenced
relations/objects are likelier to be *interesting* — they touch
more of the existing fact graph and so are more likely to either
fire useful rules or be quickly contradicted.

Note that under M1 the original "minimal-domain-first" ordering
heuristic is effectively obsolete after the post-S1.5.4 optimisations;
the scoring above is a natural successor.

**2. Branch-info ordering.** Prefer branches that *produce more
information*:

- If a branch's saturation produces **no new unconditional
  constraints**, defer going deeper inside it — first explore
  shallow alternatives.
- Before going deeper anywhere, eliminate everything that's
  shallow.

The principle: maximise the entropy reduction per branch step.
Composes with the "stable-alive" caching ([S1.5.7b](../p1.5_hypothesis_loop/s1.5.7b_consume_loop_stable_alive.md)),
where a branch that adds nothing on re-saturation is already
flagged.

### The guiding question

> How does human reasoning decide to ask first the question
> "what is the color of the first house?" in Zebra? Why not e.g.
> "what drink is drunk by the cat owner?"

Two observable patterns from the idea-08 walkthrough:

1. **All human questions for hypothesis are about houses.** Why
   houses, not nations/colours/drinks? Probably because *House* is
   the positional carrier — its 1..5 ordering grounds every other
   relation. The walkthrough's mental model has a 5-cell strip
   and fills cells; the question shape follows the model shape.
2. **Why specifically colour, and why house 1?** Conjecture: the
   smallest-domain attribute crossed with the strongest
   positional constraint. Colour has 5 values, but condition
   (10) (Norwegian in House 1) + condition (8) (Kools next to
   Horse) + condition (6) (Ivory left of Green) concentrate
   colour deductions early.

Both are *post-hoc rationalisations* of human behaviour — useful
as **score heuristics**, not as proof-of-uniqueness arguments.
The hypothesis-scoring idea above is the engine analog of #1; #2
informs the relation-priority knob.

S1.5a.7 is the home for measuring whether the scoring beats the
existing most-constrained-object first heuristic (A2 in S1.5.4's
catalog).

### Why this lives in P1.5a rather than P1.9

P1.5a is the active phase whose acceptance gate is "zebra2 solves
uniquely". The scoring and branch-info ordering are heuristics
that directly close (or fail to close) the perf budget item from
the acceptance criteria. P1.9 catalogs *follow-up* ideas that
compose with a shipped solve; these compose with *getting* solve
to work in the first place.

If S1.5a.7 lands but the heuristic doesn't help, demote to a P1.9
catalog entry (E21+) with the measurement attached.

## Approach options for (a) NAF race

Three paths, not mutually exclusive:

1. **Engine: re-evaluate NAF at fire time.** When a queued firing
   dequeues, re-run its match (or just its NAF sub-plans) against
   the current KB. If NAF now fails, drop the firing. ~15–30 LOC
   in `inference/saturator.py`'s `_apply`. Most principled. Fixes
   every NAF-on-derived-fact case at once, not just the spatial
   ones in zebra2.

2. **Per-rule "wait for closure" gating.** Defer rules whose NAF
   depends on derived facts until those facts are saturated. Needs
   a "phase marker" mechanism the engine doesn't have today.

3. **Ein-side: pre-declare derived facts.** Sidestep the race by
   manually enumerating the derived facts in `(facts)` or
   `(ontology)`. Quick + concrete (already done for next-to in
   zebra2); doesn't generalise.

(1) is the long-term answer. (3) is the M1-acceptance shortcut.

## Approach options for (b) hypothesis-count gap

1. **Hybrid encoding with co-located projection.** Re-introduce
   `co-located : Attribute × Attribute` as a derived equivalence
   relation, with `house-*` as typed projections via a
   `projection` rule. `(sibling-exclusive is-a co-located)`
   pre-emptively populates the negatives that hypgen filters on.
   The path discussed at length pre-S1.5.8c.7-spinout.

2. **linked-induced + linked-transitive.** Two-tier reasoning at
   the linked-fact level (rejected pre-spinout as functionally
   equivalent to chains the existing rules already cover; doesn't
   add unique pruning).

3. **Hypgen-side filter on linked facts.** Engine extension: have
   hypgen consult `(linked …)` activator facts directly when
   deciding which candidates to emit, rather than relying on
   `(not …)` facts to prune.

4. **Per-house-* sibling-exclusive analog.** Direct rules that
   derive `(not (house-color H_other V))` when `(house-color H V)`
   is committed and H_other ≠ H. Functional+total cover SOME of
   this already; a more aggressive version pre-emptively asserts
   the cross-house siblings.

The hybrid (1) is the most aligned with the original zebra2's
proven mechanics. (3) is engine work but lifts the abstraction
better.

## Acceptance for the phase (= M1 #2 + #3)

The phase ships when:

1. `bench_solve examples/zebra2.ein` returns
   `Solution` with `(house-nation House_? Japanese) ∧
   (house-pet House_? Zebra)` and `(house-nation House_? Norwegian)
   ∧ (house-drink House_? Water)` bound to specific houses,
   within a sensible time budget (target: < 60s on a laptop).
2. The trace contains every named rule firing the idea-08
   walkthrough mentions, in structurally equivalent order
   (matched per the M1 acceptance criterion #3 checklist —
   "Therefore the first house is yellow" → named
   `domain-elimination` firing, etc.).
3. The NAF race (or the per-rule workaround for it) is
   documented in
   [`docs/kernel/inference/`](../../../docs/kernel/inference/)
   so anyone writing new rules with derived-NAF premises knows
   the constraint.

## Cross-links

- [S1.5.8c](../p1.5_hypothesis_loop/s1.5.8c_final_decision.md) —
  the structural refactor and rule design this phase builds on.
- [M1 README — acceptance](../README.md#acceptance) — the gates
  this phase exists to close.
- [idea-08](../../../docs/ideas/08-human-style-deductive-trace.md)
  — the trace fidelity target.
- [P1.5 README](../p1.5_hypothesis_loop/README.md) — P1.5's stage
  list; this phase is a spin-out of the original S1.5.8c.7.
- [P1.9](../p1.9_hypothesis_loop_followups/) — neighboring
  follow-up phase for hypothesis-loop perf / catalog work.
- [S1.5.9](../p1.8_ein_lang_modules/s1.5.9_ein_lang_macros.md) —
  parked macro system (relocated to P1.8 Theme A 2026-05-24,
  stage id sticky); not blocking but composes.
