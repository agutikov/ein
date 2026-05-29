# Inference — the rule firing engine

> **Stub.** Becomes load-bearing when [P1.3](../../../plans/m1_core_graph_reasoning/p1.3_inference_rules/)
> ships. This page sketches the planned structure so the rest of the
> kernel tree can cross-link confidently.

The inference engine is what takes a populated
[`KnowledgeBase`](../ir/02-data-model/02_store.md) and produces
**reasoning-layer facts** by firing
[rules](../ir/01-ein-graph/02_rules.md). Everything else in the
kernel tree describes *what* the engine reads and writes; this
chapter describes *how* it does it.

---

## Planned structure (P1.3 — P1.5)

```text
docs/kernel/inference/
├── README.md         ← this file (stub)
├── 01_matcher.md     ← pattern matching: variable binding, predicate
│                       evaluation, multi-pattern conjunction
│                       (P1.3 S1.3.1)
├── 02_saturation.md  ← the firing loop; priority ordering; quiescence
│                       detection (P1.3 S1.3.3)
├── 03_hypothesis.md  ← branching: choose-an-undetermined-slot;
│                       fork + saturate + retract-on-contradiction;
│                       commit-on-uniqueness (P1.5)
├── 04_contradiction.md ← clash detection; unsat-core walk; trace
│                       generation for failed branches (P1.5)
└── 05_trace.md       ← step-by-step ein-lang trace generation;
│                       reordering pass; per-step / aggregate / DAG
│                       views (P1.6)
```

The five files map to plan phases; each ships when its phase does.

## What lives where today (M1 stubs)

| concept                         | M1 state                                                         |
|---------------------------------|------------------------------------------------------------------|
| Pattern matcher                 | `Pattern` dataclass holds structural view; matcher in P1.3       |
| Rule registry                   | `Rule` entity in [`02-data-model`](../ir/02-data-model/); rule definitions in `examples/rules.ein` (P1.3 T1.3.2.1-10) |
| Property-fact activation        | KB indexes `_rule_apps_by_rule` / `_rule_apps_on_relation` built at load |
| Saturation loop                 | not yet — placeholder in P1.3 S1.3.3                              |
| Hypothesis branching            | `kb.fork()` exists; full loop is P1.5                            |
| Contradiction detection         | `kb.unsat_core()` exists; clash detection in P1.5                |
| Trace generation                | `DerivationDAG.to_dot()` exists; markdown trace in P1.6           |
| `(not P)` / `(absent P)` premises | S1.5.8c.1 shipped: `(not P)` in `:match` matches a STORED `(not P)` fact (uniform with all other patterns); `(absent P)` is the explicit NAF guard. The old NAF default on `(not P)` was dropped. |
| `(forall ?b (G) (B))` / `(open P)` | S1.5.8c.3a/b parser sugars: `forall` desugars to `(absent (and G (absent B)))`; `open` desugars to `(and (absent P) (absent (not P)))`. No matcher addition — both compile to existing `AbsentGuard` machinery. |
| Saturator cache-refresh         | S1.5.8c-followup: `_enqueue_pass` re-runs `engine.compile_all()` at the top of each pass so runtime-derived activator facts (e.g. produced by expansion rules) get their plans compiled and the rule fires. |
| `AbsentGuard` re-evaluation at fire time | S1.5a.1 T1.5a.1.1: `Saturator._apply` calls `match.absents_still_pass(plan, bindings, kb)` before `fire()`. If a derivation between enqueue and dequeue has made an AbsentGuard's sub-plan match, the firing is dropped (counted in `Saturator.naf_dropped`). See § "NAF semantics" below. |
| Hypothesis-branch order is deterministic | S1.5a.1a T1.5a.1a.1: `solver._candidates_for` sorts by `(-score_hypothesis(f, kb), f.args, f.relation_name)`. Content-based; `PYTHONHASHSEED` does not reach the iteration order. Score is a stub (`0`) in M1; S1.5a.7 fills it. See § "Determinism" below. |

The data substrate (KB, entities, layer views, fork, provenance,
derivation DAG) is **complete** through P1.2. The engine that
*operates* on this substrate is the gap that P1.3 — P1.6 closes.

## Design principles (already locked in M1)

These are inherited from the graph + data-model docs and don't
change when the engine arrives:

1. **The graph is canonical, the engine is dynamic.**
   [`feedback_graph_canonical`](../../../README.md). The engine
   never replaces the KB; it appends to the reasoning layer.
2. **Rules can be higher-order.** Three rule types
   ([`../ir/01-ein-graph/02_rules.md`](../ir/01-ein-graph/02_rules.md));
   the matcher must enumerate relation variables.
3. **Every firing leaves provenance.** Rule-kind provenance with
   `premises_raw` and `bindings` is mandatory; trace fidelity
   ([idea 08](../../ideas/08-human-style-deductive-trace.md)) is an
   M1 acceptance gate.
4. **Lazy branching.** Saturate first with all propagation rules;
   branch only when no rule fires and the puzzle is not yet solved.
   ([Q19 working answer](../../../plans/m1_core_graph_reasoning/open_questions.md#q19).)
5. **Encoding-agnostic.** The engine must work over both `zebra.ein`
   (classic) and `zebra2.ein` (unified is-a) — the final IR encoding
   choice is deferred to P1.7. The matcher must consult
   `logical_types` / `logical_instances` where the encoding might
   matter.

## M1 invariant — `enable_alive_inherit` soundness

[`SolverConfig.enable_alive_inherit`](../../../ein.py/src/ein_bot/inference/config.py)
ships **on by default** as of S1.5.4 T1.5.4.8. With the flag on,
the hypothesis loop seeds the alive-candidate set **once at root
saturation** (via
[`generate_hypotheses_with_stats`](../../../ein.py/src/ein_bot/inference/hypgen.py))
and stashes it on `kb.alive`. Forks inherit `alive` through
[`kb.fork()`](../../../ein.py/src/ein_bot/kb/store.py); each
`_explore` entry re-prunes against the fork's KB and picks the
next hypothesis from what remains. `generate_hypotheses` runs
**once per `solve()` call**.

This is sound iff three pre-conditions hold across the puzzle's
rule library — collectively the **M1 invariant**:

1. **No new objects.** Rules don't `:assert` facts whose args
   introduce names that weren't already in the ontology /
   facts. (Q40 nested-Fact args are existing facts, not new
   names.)
2. **No new relations.** Rules don't `:assert (relation N S₀ S₁)`
   declarations — the relation registry is fixed by the ontology
   block.
3. **Hypotheses connect names only.** `_fill_slot` iterates
   `_instance_like_objects` and string-fills both slots; no
   nested-Fact hypothesis args.

Under these clauses, every admissible hypothesis is enumerable
from the root state; deeper branches **eliminate** candidates,
never extend the space.

**When the invariant breaks** (a rule library asserts new
`(relation …)`; F5 rules-as-data; a future puzzle's matcher
produces nested-Fact hypotheses):

- The default-on flag becomes **unsound** — alive entries may
  miss candidates introduced post-root.
- The escape hatch: set
  ``(config :enable-alive-inherit false)`` in the puzzle, or pass
  ``solve(kb, config=SolverConfig(enable_alive_inherit=False))``
  programmatically. The engine reverts to per-branch
  `generate_hypotheses(kb)` — the pre-`40b8dd4` shape — at the
  cost of re-enumerating every level.

Tracked at
[M1 Q-S1.5.4.D](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.4_hypgen_improvements.md#open-questions-parked-here)
as a long-term design seam; promote to a typed invariant check
when F5 lands.

## NAF semantics — fire-time re-evaluation (S1.5a.1)

`(absent P)` in a `:match` clause compiles to an
[`AbsentGuard`](../../../ein.py/src/ein_bot/inference/compile.py)
step. The matcher's
[`_run_steps`](../../../ein.py/src/ein_bot/inference/match.py)
yields a binding only when the AbsentGuard's `sub_steps` produce
zero matches against the current KB — classical NAF over the
saturator's accumulating fact base.

**The race.** The saturator's enqueue pass evaluates every plan
step (including AbsentGuards) when it admits a candidate firing
to the priority queue. The firing then sits in the heap until its
priority comes up. Between enqueue and dequeue, other rules may
have derived new facts — and one of those facts may now satisfy
an AbsentGuard's sub-plan, retroactively invalidating the NAF
verdict that admitted the firing. Without a fire-time check the
saturator commits the firing anyway, producing an *unsound*
derivation against the closed KB.

Priority ordering hides the race when every rule that *derives*
the watched relation runs at a strictly-lower priority than every
rule that *NAFs* it. zebra2's `(includes right-of next-to)` +
`(symmetric next-to)` chain runs at priority 100, fully draining
before priority-200 cross-attr rules with NAFs over `next-to` are
ever enqueued — so the race is structurally prevented for that
shape. Bands ≥ 200 that derive facts another band-≥ 200 rule
NAFs over don't have that protection, and neither does any
branched saturation that starts with a non-empty queue.

**The fix.**
[`match.absents_still_pass(plan, bindings, kb)`](../../../ein.py/src/ein_bot/inference/match.py)
walks the plan's top-level `AbsentGuard` steps and re-runs each
sub-plan against the *current* KB with the dequeued bindings.
`Saturator._apply` calls it after the redundant-conclusion check
and before
[`fire()`](../../../ein.py/src/ein_bot/inference/firing.py);
on `False`, the binding is recorded in `engine._fired` (so the
queue stops churning on it), `Saturator.naf_dropped` is
incremented, and `_apply` returns `None`. The caller in `step()`
treats `None` as "skip and pop again."

Nested AbsentGuards (e.g. from a `forall` desugar to
`(absent (and G (absent B)))`) compose transparently — the outer's
`sub_steps` flow through `_run_steps`, which recurses on the inner
AbsentGuard against the same current KB. Only AbsentGuards are
re-checked: `Scan`/`Join` steps can only narrow under monotonic
fact growth, and `Guard` predicates are stateless over the KB.

**Termination.** Within a single `saturate()` run the fact base
grows monotonically (no retractions). Once an AbsentGuard fails
at fire time, the watched fact it tripped on stays in the KB; the
binding sits in `_fired` and is not re-enqueued. A dropped firing
removes itself from the fixpoint candidate set without re-entering
— termination is preserved.

The retracting flow that *does* exist (hypothesis branching's
`kb.fork()`) takes a fresh saturator over the branch KB; the
branch starts with an empty `_seen`/`_queue` and inherits no
dropped-firing state from the parent. The branched saturator
re-evaluates every plan against its own KB and so its
`naf_dropped` count is independent.

**Open follow-ups.**

- [Q-S1.5a.1.B](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.1_naf_semantic_rearch.md#open-questions)
  — caching per-(rule, binding) NAF results and invalidating on
  watched-fact arrival. Composes with
  [P1.9 E8](../../../plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/)
  (watched-fact rule applicability).
- [S1.7.4](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md)
  — static NAF dependency map; emit a load-time warning when a
  derived relation is the target of an AbsentGuard, so authors
  know which rules rely on the fire-time check. Relocated to
  P1.7 on 2026-05-26 (formerly P1.5a S1.5a.8 / T1.5a.1.2).

## Hypgen pre-pruning — disjunctive-prune (S1.5a.2)

The hypothesis generator
([`generate_hypotheses`](../../../ein.py/src/ein_bot/inference/hypgen.py))
emits one candidate `(?R ?A ?B)` per legal slot-fill at root
saturation; each candidate becomes a hypothesis the solver
might branch on. The generator's filter consults
[`kb._negated_facts`](../../../ein.py/src/ein_bot/kb/store.py)
to drop candidates whose negation is already known: a
candidate ``(color-loc Yellow House-3)`` is dropped if
``(not (color-loc Yellow House-3))`` is in `_negated_facts`.
The `--hyp-stats` output's `filtered.negated_fact` counter
measures this filter's hit-rate (62 of 125 raw candidates on
zebra2, leaving 56 emitted).

Pre-S1.5a.2 the only `(not …)` facts entering
`_negated_facts` at root saturation were the ones the puzzle
declared directly. Cross-attribute spatial constraints
(`adjacent-via`) didn't contribute negatives — the
`adjacent-via-{fwd,bwd}` rules only assert positives when the
spatial neighbour is unique. For non-corner houses no positive
fires and the candidate stays in the hypothesis space.

**The fix.** Two new rules ship with each `adjacent-via`
activator:

- ``disjunctive-prune-fwd ?S ?R1 ?V1 ?R2 ?V2`` — given
  ``(R1 V1 h1)``, for every ``h_other`` in the partner's
  type-domain where ``(?S h_other h1)`` is absent, assert
  ``(not (R2 V2 h_other))``.
- ``disjunctive-prune-bwd`` — symmetric, with the NAF operand
  order reversed for asymmetric ?S like ``right-of``.

These fire in BOTH unique and non-unique cases, so they
contribute negatives even when the positive can't be pinned.
The pair derives from a single ``(adjacent-via ?S V1 V2)``
activator via two meta-rules (`derive-disjunctive-prune-{fwd,bwd}`
at priority 200) — author writes one activator, gets both
pre-pruners. Priority 250 on the pruner itself ensures the
next-to derivations at priority 100 drain first, so the NAF
guard sees the closed adjacency graph.

The split into `-fwd` / `-bwd` matters for asymmetric ?S:
pre-S1.5a.2 there was a single rule whose `-bwd` direction
swapped the activator args but kept the `-fwd` NAF, asserting
spurious ``(not (color-loc Ivory House-4))`` from a known
``Green@House-5``. For symmetric ``next-to`` the two NAF
directions are equivalent and the bug was masked; the
S1.5a.11 dump on `zebra2-hints.ein` surfaced it in its first
realistic outing. The two-rule structure makes each
direction's NAF explicit in its own match clause.

## Determinism — content-based candidate ordering (S1.5a.1a)

`solve()` visits hypothesis branches in the order
[`_candidates_for`](../../../ein.py/src/ein_bot/inference/solver.py)
returns them. Pre-S1.5a.1a that list was the iteration of a
`frozenset` (the root alive-set stashed on `kb.alive`), which
reaches `hash(Fact)`, which reaches `hash(str)` — randomised
per process by Python since 3.3. The visible symptom: every
`bench_solve` invocation explored branches in a different order.

The fix sorts the result of `_candidates_for` by
[`_candidate_sort_key`](../../../ein.py/src/ein_bot/inference/solver.py):

```python
(-score_hypothesis(fact, kb), fact.args, fact.relation_name)
```

All three components are content-derived; `hash(str)` never
reaches the tuple. With the M1 stub
[`score_hypothesis`](../../../ein.py/src/ein_bot/inference/hypgen.py)
returning `0` for every fact, the effective order is
``(args, relation_name)`` — alphabetic on first arg, then
second, then relation. The score primary key is the slot
[S1.5a.7](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.7_hypgen_scoring_branch_info.md)
fills in (fact-popularity sum, weighted relation/object
coefficients); when it lands, the solver doesn't move.

**Determinism rule for new code.** Any `set` / `frozenset`
whose iteration order influences user-visible output (branch
IDs, trace ordering, log lines, fixture-dependent test
assertions) must be sorted at the iteration boundary. `set`
membership checks, `_fired`, `_negated_facts`, `_seen` — these
are membership-only and don't need sorting. The audit point is
the read site, not the storage site.

[`tests/inference/test_branch_determinism.py`](../../../ein.py/tests/inference/test_branch_determinism.py)
spawns two subprocesses with different `PYTHONHASHSEED` and
asserts their solve output is byte-identical; any regression
that re-leaks hash order into the candidate path fails the
subprocess test.

## d=0 negative-completion (S1.5a.19)

The NL Zebra walkthrough closes at depth 0 — every "Therefore X"
in the trace is reachable from the puzzle's facts + ontology
without any hypothesis branching. Pre-S1.5a.19, the engine
needed branching to discover the same negatives: a known
``(color-loc Yellow House-1)`` did not derive
``(not (color-loc Yellow House-{2,3,4,5}))`` in the same
saturation pass, so the candidates lingered in hypgen's
output and the solver split into 568 nodes searching for a
contradiction that NL closes at d=0. After S1.5a.19 the tree
collapses to 32 nodes at `--max-depth 1` (see
[`STATUS.md`](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/STATUS.md)).

Six new rules ship in
[`examples/zebra2.ein`](../../../examples/zebra2.ein) (mirrored
in `zebra2-hints.ein`) to close the gap. Each derives a
``(not …)`` directly from positive evidence + an ontology
declaration, with no recourse to branching:

| rule                                              | premise pattern                                                                                                                 | derived negative                                |
|---------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------|
| `functional-negative ?R`                          | ``(R ?a ?b)`` ∧ ``functional R`` ∧ ``b' ≠ b``                                                                                  | ``(not (R a b'))``                              |
| `injective-negative ?R`                           | ``(R ?a ?b)`` ∧ ``injective R`` ∧ ``a' ≠ a``                                                                                   | ``(not (R a' b))``                              |
| `co-located-negative ?R1 ?V1 ?R2 ?V2`             | ``(co-located R1 V1 R2 V2)`` ∧ ``(not (R1 V1 h))``                                                                             | ``(not (R2 V2 h))``                             |
| `adjacent-via-endpoint-fwd ?S ?R1 ?V1 ?R2 ?V2`    | no ``h2`` with ``(?S h2 h1)``                                                                                                  | ``(not (R1 V1 h1))``                            |
| `adjacent-via-endpoint-bwd`                       | no ``h1`` with ``(?S h2 h1)``                                                                                                  | ``(not (R2 V2 h2))``                            |
| `adjacent-via-{fwd,bwd}-negative`                 | contrapositive of `adjacent-via-{fwd,bwd}` — ``(not (R2 V2 h2))`` + unique ?S-source ⟹ ``(not (R1 V1 h1))`` (and symmetric)    | ``(not (R1 V1 h1))`` resp. ``(not (R2 V2 h2))`` |

Each rule has a `derive-…` meta-rule (priority 100 or 200)
that lifts an ontology activator (`functional R`,
`co-located R1 V1 R2 V2`, `adjacent-via-{fwd,bwd} ?S …`) into
the target rule's own activator. Authors keep writing one
ontology-level declaration per constraint; the engine fans it
out into the negative-completion machinery automatically.

**Priority discipline** (lines 117-120 of `examples/zebra2.ein`):
the negative rules sit at priority 240 — AFTER propagation
(200) so the new positives are visible, BEFORE both
sibling-violation rules (250) and elimination rules (400), so
derived negatives reach `domain/range-elimination`'s `forall`
premises in the same pass.

The NL chain this closes (see
[`examples/README.md`](../../../examples/README.md)):
*Norwegian@H_1 ⟹ Englishman ≠ H_1 ⟹ Red ≠ H_1* — once
`functional-negative` produces the first negative,
`co-located-negative` propagates it across the equivalence,
and the cascade terminates at the corner-house exclusions from
`adjacent-via-endpoint-{fwd,bwd}`.

Naming convention: every rule name carries `-negative` so the
trace renderer (P1.6 S1.6.4) can group derivation events by
polarity. The `derive-…-negative` meta-rules are similarly
named after the target they enable.

## Mid-sweep saturation + per-sibling apriori re-check (S1.5a.19)

The d=0 rules above are necessary but not sufficient on their
own — the solver's `_consume` loop must actually *use* the new
negatives. Pre-S1.5a.19's loop tested every sibling in the
parent's alive set via the full `try_branch` (fork + saturate
+ contradiction-detect), even if an earlier sibling's
back-prop had just made the next sibling apriori dead. The
cost was paid for the contradiction to re-surface inside the
fork.

S1.5a.19 fixes this with two cheap pre-fork checks plus a
mid-sweep saturator pass
([`solver.py:1075-1122`](../../../ein.py/src/ein_bot/inference/solver.py)):

```python
for h in to_check:
    key = (h.relation_name, h.args)
    # (a) Apriori Tier-A re-check: earlier sibling's back-prop
    #     + in-sweep re-saturation may have made h dead.
    if key in kb._negated_facts:
        stats.apriori_dead_in_sweep += 1
        # mark dead, no try_branch
        continue
    # (b) Mid-sweep re-saturation may have derived h's positive
    #     directly (functional / adjacency closure).
    if kb._fact_by_id(h.relation_name, h.args) is not None:
        # mark alive, no try_branch
        continue
    result = try_branch(kb, h, branch_id=...)
    if result.is_alive():
        ...
    elif is_unconditional_death(result.kb, result.unsat_core, ...):
        back_propagate(kb, h, result.unsat_core)
        # Mid-sweep saturator: propagate (not h)'s transitive
        # consequences into kb so subsequent apriori re-checks
        # can skip more siblings.
        mid_sweep_firings.extend(
            Saturator(kb).saturate(max_steps=10_000))
    else:
        ...
```

Three pieces compose:

1. **Apriori Tier-A re-check** before `try_branch`: query
   `kb._negated_facts` directly; if the sibling's negation is
   now known, skip the fork and mark dead in one O(1) step.
   Counted in `stats.apriori_dead_in_sweep`.
2. **Positive-already-derived check**: between siblings the
   mid-sweep saturator may have derived h's positive directly
   (e.g. via `adjacent-via-bwd` from a recently-pinned
   ``(?R2 ?V2 h2)``); mark alive and skip the fork.
3. **Mid-sweep `Saturator(kb).saturate(...)`** after each
   `back_propagate`: runs the saturator on the parent KB with
   the freshly-bubbled ``(not h)`` so the d=0
   negative-completion rules can fire transitively before the
   next sibling is tested. `max_steps=10_000` caps the cost;
   on zebra2 the sweep terminates well below the cap.

Measured impact on zebra2 (depth 1): 28 of 31 dead leaves
(`apriori_dead_in_sweep=28`) skip `try_branch` entirely via
the Tier-A path; the three remaining dead siblings need the
full fork (cases the apriori check can't predict from
`_negated_facts` alone — e.g. a sibling whose conditional
contradiction depends on the candidate's own consequences).

Together the rules + mechanism implement at the engine level
what the NL trace does at the cognitive level: each commitment
unfolds its consequences fully before the next decision. The
result is the 568 → 32 node collapse documented in
[`STATUS.md`](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/STATUS.md).

**Future composition.** The mid-sweep saturator pass is the
engine's "go up" channel; pre-2026-05-26 it was the motivation
for the now-dropped [S1.5a.20 branch-isolation re-architecture](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.20_branch_isolation_rearch.md).
The
[P1.5b](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/)
set-indexed engines (monotonic + lattice) bake the per-set
saturate-from-root pattern in from the start, so the mid-sweep
pass becomes the default control flow rather than an opt-in.
The tree-side `_consume` keeps the explicit mid-sweep until
P1.5b reaches parity; then the per-sibling re-check moves to
whichever engine inherits the responsibility.

## Unconditional death — back-prop soundness (S1.5.7)

When a hypothesis branch dies, the engine asks whether the death is
**unconditional** — whether the contradiction would recur from the
parent KB's facts alone, with the branch's hypothesis `h` playing no
part. An unconditional death licenses back-propagating `(not h)` into
the parent ([S1.5.7](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md)),
where it becomes an O(1) `_negated_facts` filter entry every sibling
and descendant inherits.

The test is **not** the shallow one — *"the unsat-core contains no
`kind='hypothesis'` fact"*. That read is unsound: an unsat-core fact
can be `kind='rule'` and still derive, through a chain of firings,
from a hypothesis — its own provenance is `'rule'`, but its premises
are not. [`back_prop.reaches_hypothesis`](../../../ein.py/src/ein_bot/inference/back_prop.py)
walks `Provenance.premises_raw` transitively — resolving each id
against the KB — until every chain grounds out at a `source`-kind /
un-provenanced given, or any chain reaches a `hypothesis` / `rejected`
terminal. `is_unconditional_death(kb, unsat_core)` is True iff no
chain reaches a speculative terminal.

The asymmetry is load-bearing: a missed unconditional death merely
forgoes a cache entry, but a *false* one writes `(not h)` into the
parent irreversibly and wrongly excludes a valid hypothesis. The
predicate therefore errs conditional — an empty or unresolvable
unsat-core reads as conditional, never unconditional.

## Set-indexed search — monotonic engine (P1.5b S1.5b.0–.10)

The tree engine's depth-first ordering over hypothesis branches
prices in d! orderings of the same commitment set — for d=4 on
zebra2 that's 24× redundant work on each set. The **monotonic
engine** under
[`ein.py/src/ein_bot/inference/monotonic/`](../../../ein.py/src/ein_bot/inference/monotonic/)
collapses this by indexing by commitment **set** rather than
path: layer N enumerates every size-N alive subset via
Apriori-style prefix-join, enters each via the common
[`try_commitment_set`](../../../ein.py/src/ein_bot/inference/commitment.py)
primitive, and merges only the unconditional consequences back
into a single root KB. First goal-satisfying commitment wins
(SOLVE mode — Q1.5b.7); GAPS / CONTRADICTIONS belong to the
peer **lattice engine** (S1.5b.20+).

### Termination conditions, in order of precedence

1. **Solution at a fork.** `is_solved(result.kb, mode)` on an
   alive entering — the fork's saturated kb carries the
   committed hypotheses + their derivations, which is the
   context the goal needs when it references hypothesis facts
   directly (e.g. `examples/branching/05_mini_zebra.ein`).
   Returns `Solution(kb=result.kb)` so the caller sees the
   hypothesis context the goal depended on. **Algorithm spec
   §3d.vii.**
2. **Solution at root.** After merging unconditional facts from
   an alive commitment, a forced-positive cascade may promote
   remaining singleton hypotheses to root and `is_solved` fires
   there. This is how `examples/zebra2.ein` solves —
   `(color-loc Green House-4)` cascades into 30 unconditional
   facts that complete the puzzle at root via the chain of
   pre-emptive lookahead negatives.
3. **Contradiction at Phase 1.** Root saturates to `(false)` —
   the puzzle is inconsistent before any hypothesis enters.
4. **Contradiction at Phase 3.** Every layer-1 singleton died;
   `_compute_alive` returns ∅; verdict is Contradiction.
5. **Ambiguity.** Layer cap reached with alive ≠ ∅ and no
   goal-satisfying commitment found.

### CDCL nogoods (S1.5b.6)

Every dead entering emits `frozenset(C)` into
`root_kb._nogoods` via `inference.nogoods.emit_nogood`
(min_size=1 so layer-1 singleton deaths land — Q1.5b.5.c).
The next layer's `generate_layer` filters supersets via the
existing `apriori.filter_candidate` subset check; the engine
never re-enters a strict superset of a known-dead set.
Singleton dead clauses additionally write `(not h)` into
`root_kb._negated_facts` (plus the symmetric mirror if
`(symmetric R)` is in the ontology) so subsequent
`_compute_alive` calls drop h from `alive`.

### Diagnostics — `MonotonicDumper` (S1.5b.7)

Optional `dumper=MonotonicDumper(out_dir=…)` captures:

```
dump/<puzzle>-<ts>/
   00_root_initial.ein           ← root before any enterings
   00_timeline.jsonl             ← chronological event log
   layers/
       layer_NN_pre.ein          ← root.kb at layer N start
       layer_NN_post.ein         ← root.kb at layer N end
   summary.json                  ← final stats + verdict
```

Six lifecycle hooks (`root_initial`, `layer_start`, `entering`,
`layer_end`, `early_terminate`, `summary`) fire from the
backbone; `dumper=None` is a no-op for every hook site. The
`_VerboseDumper` subclass in `demo/bench_monotonic.py` streams
the same events to stderr as `--verbose` progress lines without
needing an on-disk dump.

`MonotonicDumper` captures only the per-layer root snapshots —
the solution-mode engine early-terminates, so most hypotheses are
never reached and there's nothing per-hypothesis to record. For a
**complete per-hypothesis record** — every commitment tested at
every layer, with the firings each one emitted, survivors and
casualties alike — run the exhaustive lattice entries
(`gaps_solve` / `contradictions_solve`) with a `LatticeDumper`:
see [`lattice_dump.md`](lattice_dump.md). That dump groups
`enterings/` and `kb_index/` by layer and writes
`outcome.txt` + `firings.jsonl` + `unsat_core.jsonl` per
commitment — the audit trail for debugging problem statements and
rules.

### Budget — `max_time` / `max_enterings`

`monotonic_solve(..., max_time=N, max_enterings=K)` checks the
caps before every `try_commitment_set` call; on exhaust raises
`BudgetExceededError(reason, stats)` with the partial counters.
The dumper's timeline is flushed via `MonotonicDumper.close()`
on the abort path (no `summary.json` then — the events up to
the abort suffice for diagnostic).

### Measured performance

On the laptop reference (PyPy):

- `examples/zebra2.ein`: Solution in ~1.9 s (CPython ~2.8 s),
  1 alive entering, 0 nogoods — single-shot solve via fork-side
  `is_solved`. ~18× faster than tree on CPython; ~4× on PyPy.
- `examples/branching/*` (11 fixtures): all 11 reach the
  tree-side bindings; combined parity-test wall ~3.5 s. See
  [`parity_baselines.md`](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/parity_baselines.md).

### Cross-links

- Stage plan: [P1.5b S1.5b.0–.10 in p1.5b_lattice_search/](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/).
- Equivalence claim: [Q1.5b.7 § monotonic vs lattice](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/open_questions.md#q15b7--termination--completeness--mode-handling).
- Algorithm spec: [`algorithm_layer_n.md`](../../../plans/m1_core_graph_reasoning/p1.5b_lattice_search/algorithm_layer_n.md) §3d.

## Where the design lives today

The complete plan, including task breakdown and acceptance criteria:

- Plan phase [P1.3 — Inference rules](../../../plans/m1_core_graph_reasoning/p1.3_inference_rules/).
- Plan phase [P1.5 — Hypothesis loop](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/).
- Plan phase [P1.6 — Rendering + trace](../../../plans/m1_core_graph_reasoning/p1.6_rendering_and_trace/).
- Idea: [`docs/ideas/06-inference-rules-completeness.md`](../../ideas/06-inference-rules-completeness.md).
- Idea: [`docs/ideas/08-human-style-deductive-trace.md`](../../ideas/08-human-style-deductive-trace.md).

When P1.3 work begins, this stub becomes a hub for the
implementation reality.
