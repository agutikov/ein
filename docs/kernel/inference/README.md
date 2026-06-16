# Inference — the rule firing engine

> **Status (2026-06-16).** The engine has **shipped** — P1.3
> (saturation / rules), P1.4 (contradiction), P1.5–P1.5b (hypothesis
> loop + commitment-lattice search) are all in place and `zebra2`
> solves end-to-end. The **as-built** architecture reference is
> [`architecture_and_algorithms.md`](architecture_and_algorithms.md);
> this file holds the design principles, the M1 invariant, NAF
> semantics, and determinism. The "as-built layout" + "what's
> implemented" sections were reconciled by
> [P1.20 S1.20.A0](../../../plans/m1_core_graph_reasoning/p1.20_kernel_docs/s1.20.a0_reconcile_drift.md);
> a deeper module-level walkthrough — and the still-stale "Determinism"
> / "Superseded tree-solver" sections lower down (broken
> `inference/solver.py` links; the file was split into `monotonic/`) —
> is [S1.20.D](../../../plans/m1_core_graph_reasoning/p1.20_kernel_docs/s1.20.d_inference_engine_docs.md)'s remit.

The inference engine is what takes a populated
[`KnowledgeBase`](../ir/02-data-model/02_store.md) and produces
**reasoning-layer facts** by firing
[rules](../ir/01-ein-graph/02_rules.md). Everything else in the
kernel tree describes *what* the engine reads and writes; this
chapter describes *how* it does it.

> **Architecture + algorithms overview.** For the engine's *as-built*
> architecture and main steps, the abstract operations it performs, their
> analogs in other CS fields (Datalog / RETE / CDCL / ATMS / e-graphs), and
> the fast/optimal known algorithms for each, see
> [`architecture_and_algorithms.md`](architecture_and_algorithms.md) — the
> overview the planned chapters below sit under.

---

## As-built layout

The engine shipped across P1.3–P1.5b; the planned `01_matcher.md …
05_trace.md` split was never created — the directory instead grew
these reference docs:

```text
docs/kernel/inference/
├── README.md                      ← this file: design principles, the
│                                     M1 invariant, NAF semantics, determinism
├── architecture_and_algorithms.md ← the as-built architecture: the 9 core
│                                     operations (O1–O9), their CS analogs
│                                     (Datalog / RETE / CDCL / ATMS), fast algos
├── domain_elim_vs_hypothesis.md   ← the domain-elimination vs guess duals
├── lattice_dump.md                ← the commitment-lattice dump format
├── reserved_engine_strings.md     ← engine-internal reserved atoms
│                                     (__closed__, __symmetric__, false, …)
└── python_impl.md                 ← the file-by-file Python module map (S1.20.D)
```

Source for the engine lives under
[`ein.py/src/ein/inference/`](../../../ein.py/src/ein/inference/) (+ the
`monotonic/` lattice-search sub-package); a flattened module-level
walkthrough is [S1.20.D](../../../plans/m1_core_graph_reasoning/p1.20_kernel_docs/s1.20.d_inference_engine_docs.md)'s deliverable.

## What's implemented (M1)

The engine shipped end-to-end; `examples/zebra2.ein` solves with a full
derivation trace. Where each piece lives (`ein.py/src/ein/inference/`
unless noted):

| concept                         | M1 state                                                         |
|---------------------------------|------------------------------------------------------------------|
| Pattern matcher                 | **shipped** — `compile.py` lowers each (rule, activator) to a `JoinPlan`; `match.py` `_run_steps` executes it |
| Rule registry                   | `Rule` entity in [`02-data-model`](../ir/02-data-model/); puzzle rules authored inline or imported from the [stdlib](../../../ein.py/src/ein/stdlib/) |
| Property-fact activation        | KB indexes `_rule_apps_by_rule` / `_rule_apps_on_relation` built at load |
| Saturation loop                 | **shipped** — priority-banded, delta-driven `saturator.py` (P1.3 S1.3.3; semi-naive in P1.8a) |
| Hypothesis branching            | **shipped** — `hypgen.py` enumerates candidates; the commitment-lattice search is `monotonic/solver.py` (P1.5–P1.5b) |
| Contradiction detection         | **shipped** — `contradiction.py` (`(X, ¬X)` pairs + `(false)`); minimal unsat core via `min_core.py` + provenance |
| Verdict                         | **shipped** — one `solve()`; `verdict.py` reports `Solution` / `Ambiguity` / `Contradiction`, read off the model count `k` |
| Trace generation                | **shipped** — `DerivationDAG.to_dot()` + the markdown trace builder under [`trace/`](../../../ein.py/src/ein/trace/) (P1.6) |
| `(not P)` / `(absent P)` premises | S1.5.8c.1: `(not P)` in `:match` matches a STORED `(not P)` fact (uniform with all other patterns); `(absent P)` is the explicit NAF guard. The old NAF default on `(not P)` was dropped. |
| `(forall ?b (G) (B))` / `(open P)` | S1.5.8c.3a/b sugars (now in `std.macro`): `forall` ⇒ `(absent (and G (absent B)))`, `open` ⇒ `(and (absent P) (absent (not P)))`. Compile to existing `AbsentGuard` machinery. |
| `AbsentGuard` re-evaluation at fire time | S1.5a.1: `Saturator._apply` calls `match.absents_still_pass(plan, bindings, kb)` before `fire()`. A firing whose NAF a later derivation invalidated is dropped (`Saturator.naf_dropped`). See § "NAF semantics" below. |
| Hypothesis-branch order is deterministic | content-based sort, not hash-based (`PYTHONHASHSEED` does not reach iteration order); the score key is an M1 stub. See § "Determinism" below. |

The data substrate (KB, entities, layer views, fork, provenance,
derivation DAG) was complete at P1.2; the engine that *operates* on it
shipped across P1.3–P1.6.

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
5. **Encoding-agnostic.** The engine works over both `zebra.ein`
   (classic) and `zebra2.ein` (unified is-a). P1.7 resolved the
   encoding (`is-a` canonical) and S1.7.23 removed the kernel
   type/instance entity-view, so the engine treats `(type …)` /
   `(instance …)` / `is-a` uniformly as facts — there is no
   `logical_types` / `logical_instances` bridge to consult.

## M1 invariant — alive-set soundness

The solver **recomputes** the alive-candidate set per-KB via
[`_compute_alive`](../../../ein.py/src/ein/inference/monotonic/solver.py)
(`= open_hypotheses(kb)`) — the open hypotheses are a pure function of the
closed KB. (Historical: an *inherit-once* optimization — seed `kb.alive` at
root saturation, let forks inherit it via `kb.fork()`, run
`generate_hypotheses` once per `solve()` — was gated by a
`SolverConfig.enable_alive_inherit` flag, default on since S1.5.4 T1.5.4.8.
P1.7a switched to the per-KB recompute and the flag was **removed 2026-06-15**.)

That "alive is a pure function of the closed KB" property is sound iff three
pre-conditions hold across the puzzle's rule library — collectively the
**M1 invariant**:

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
from the current KB state; deeper branches **eliminate** candidates,
never extend the space. The same "alive ⇐ KB" argument licenses the
[`canon.state_hash`](../../../ein.py/src/ein/inference/canon.py)
KB-only dedup — two KBs with identical facts have identical futures.

**When the invariant breaks** (a rule library asserts new
`(relation …)`; F5 rules-as-data; a future puzzle's matcher
produces nested-Fact hypotheses), "alive is a pure function of the
closed KB" no longer holds, and both the per-KB recompute and the
state_hash dedup lose their soundness warrant.

Tracked at
[M1 Q-S1.5.4.D](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.4_hypgen_improvements.md#open-questions-parked-here)
as a long-term design seam; promote to a typed invariant check
when F5 lands.

## NAF semantics — fire-time re-evaluation (S1.5a.1)

`(absent P)` in a `:match` clause compiles to an
[`AbsentGuard`](../../../ein.py/src/ein/inference/compile.py)
step. The matcher's
[`_run_steps`](../../../ein.py/src/ein/inference/match.py)
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
[`match.absents_still_pass(plan, bindings, kb)`](../../../ein.py/src/ein/inference/match.py)
walks the plan's top-level `AbsentGuard` steps and re-runs each
sub-plan against the *current* KB with the dequeued bindings.
`Saturator._apply` calls it after the redundant-conclusion check
and before
[`fire()`](../../../ein.py/src/ein/inference/firing.py);
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

**Static NAF dependency map (S1.7.4).** The fire-time re-eval makes
*every* derived-NAF rule sound, but it doesn't tell the author *which*
of their rules rely on it. [`naf_deps`](../../../ein.py/src/ein/inference/naf_deps.py)
answers that statically:
[`Engine.naf_dependency_map()`](../../../ein.py/src/ein/inference/engine.py)
walks the compile cache and returns one `NafDep` per `(rule, activator)`
that carries an `AbsentGuard`, splitting the watched relations into
`derived` (some rule positively asserts it — or, for an
`(absent (not (R …)))` guard, some rule asserts `(not (R …))`) vs
`declared_only` (extension fixed by enumerated ONTOLOGY-/FACT-layer
facts — no rule produces it). The
classification reuses [`compile.asserted_relation`](../../../ein.py/src/ein/inference/compile.py)
(the same test behind [`closed.producible_relations`](../../../ein.py/src/ein/inference/closed.py))
and its `negated_relation` dual. Because the activator-bound head var
(`?S` in `adjacent-via-*`) is baked to a literal relation per activator,
the split is per-activator: zebra2's `adjacent-via-fwd` is derived-NAF on
its `next-to` activator but declared-only on `right-of` — mirroring the
priority-protection note above. **The map is only complete on a
post-initial-saturation cache** — most NAF-bearing rules (the spatial and
elimination families) are activated by *derived* facts absent at load —
so the warning is emitted once, after `_phase1_root`'s root saturation,
gated by `SolverConfig.warn_derived_naf` (a `DerivedNafWarning`). That
flag defaults **off**: while `closed` stays hardcoded the NAF is sound
regardless, and the suite runs under `filterwarnings=["error"]`; it
promotes to load-bearing under [S1.7.7](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.7_kernel_purity_analysis.md).

**Open follow-ups.**

- [Q-S1.5a.1.B](../../../plans/m1_core_graph_reasoning/p1.5a_zebra_solution/s1.5a.1_naf_semantic_rearch.md#open-questions)
  — caching per-(rule, binding) NAF results and invalidating on
  watched-fact arrival. Composes with
  [P1.9 E8](../../../plans/m1_core_graph_reasoning/p1.9_hypothesis_loop_followups/)
  (watched-fact rule applicability).
- [S1.7.4](../../../plans/m1_core_graph_reasoning/p1.7_bootstrapping_zebra/s1.7.4_naf_dependency_map.md)
  — static NAF dependency map: **shipped** 2026-06-01 (see "Static
  NAF dependency map" above). `Engine.naf_dependency_map()` +
  the post-saturation `DerivedNafWarning` (default-off
  `warn_derived_naf`). Relocated to P1.7 on 2026-05-26 (formerly
  P1.5a S1.5a.8 / T1.5a.1.2).

## Hypgen pre-pruning — disjunctive-prune (S1.5a.2)

The hypothesis generator
([`generate_hypotheses`](../../../ein.py/src/ein/inference/hypgen.py))
emits one candidate `(?R ?A ?B)` per legal slot-fill at root
saturation; each candidate becomes a hypothesis the solver
might branch on. The generator's filter consults
[`kb._negated_facts`](../../../ein.py/src/ein/kb/store.py)
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

> **Reconcile note.** The names in this section are the **removed tree
> solver's** (`_candidates_for` / `_candidate_sort_key`, old
> `inference/solver.py`). The live ordering is
> [`apriori.order_candidates`](../../../ein.py/src/ein/inference/apriori.py)
> (+ `_set_score`), applied by
> [`monotonic/solver.py`](../../../ein.py/src/ein/inference/monotonic/solver.py);
> [`hypgen.score_hypothesis`](../../../ein.py/src/ein/inference/hypgen.py) is the
> score key. The *principle* below — sort candidates by a content key, never by
> `frozenset` / hash iteration order — is unchanged.

`solve()` visits hypothesis branches in the order
`_candidates_for`
returns them. Pre-S1.5a.1a that list was the iteration of a
`frozenset` (the root alive-set stashed on `kb.alive`), which
reaches `hash(Fact)`, which reaches `hash(str)` — randomised
per process by Python since 3.3. The visible symptom: every
`bench_solve` invocation explored branches in a different order.

The fix sorts the result of `_candidates_for` by
`_candidate_sort_key`:

```python
(-score_hypothesis(fact, kb), fact.args, fact.relation_name)
```

All three components are content-derived; `hash(str)` never
reaches the tuple. With the M1 stub
[`score_hypothesis`](../../../ein.py/src/ein/inference/hypgen.py)
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

These elimination rules are pathway **A** in the "domain-elimination
rule vs explicit hypothesis exploration" comparison: when their
premises are derivable they solve at root saturation and preempt the
hypothesis search entirely. The S1.5b.32 measurement
([`domain_elim_vs_hypothesis.md`](domain_elim_vs_hypothesis.md))
quantifies the trade-off — A leaves the exhaustive lattice at 0 sets;
without it the engine falls back to forced-positive promotion (1 set)
or, with every elimination path off, full branch-and-refute (7 sets,
6 nogoods on the fixture).

## Mid-sweep saturation + per-sibling apriori re-check (S1.5a.19)

> **Superseded — describes the removed tree solver** (tree engine removed in
> `8d77b02`; its last dead residue, `back_prop.py`, deleted in S1.9.E6a). The
> `inference/solver.py` `_consume` loop, `try_branch`, `back_propagate`, and
> `is_unconditional_death` named below no longer exist. The live engine is the
> set-indexed **lattice** (see *Set-indexed search — monotonic engine* below),
> which bakes the per-set saturate-from-root pattern in from the start; the
> transitive unconditional walk now lives in `commitment._is_unconditional`.
> Kept for the algorithmic intuition: each commitment closes its consequences
> before the next decision.

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
(`solver.py:1075-1122`, the removed file):

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

## Unconditional facts — `_is_unconditional` soundness (S1.5.7 / S1.9.E6a)

When a commitment's fork saturates, the engine asks of each newly-derived
fact whether it is **unconditional** — derivable from the root KB's facts
alone, with no committed hypothesis playing any part
([S1.5.7](../../../plans/m1_core_graph_reasoning/p1.5_hypothesis_loop/s1.5.7_back_prop_unconditional.md)).
Unconditional facts are merged into the root
(`try_commitment_set`'s `unconditional_facts`), where they monotonically
shrink the alive set every later layer inherits.

The test is **not** the shallow one — *"the fact's own provenance isn't
`kind='hypothesis'`"*. That read is unsound: a `kind='rule'` fact can still
derive, through a chain of firings, from a committed hypothesis — its own
provenance is `'rule'`, but its premises are not.
[`commitment._is_unconditional`](../../../ein.py/src/ein/inference/commitment.py)
runs the shared
[`provenance.reaches`](../../../ein.py/src/ein/kb/provenance.py) DFS over
`premises_raw` transitively — resolving each id against the KB — with a
*commitment-set terminal*: a chain is **conditional** iff it reaches a
`FactId` in the committed set, and **unconditional** iff every chain grounds
out at root facts first.

The asymmetry is load-bearing: a missed unconditional fact merely forgoes a
root merge, but a *false* one promotes a hypothesis-dependent fact to the root
irreversibly. The predicate therefore errs conditional — an empty or
unresolvable chain reads as conditional, never unconditional.

The negative dual — caching a forced `(not h)` — is deliberately narrower:
only the **one-step lookahead kill** (`hypgen._write_negated`, gated by
`enable_lookahead_kill_cache`) writes a `(not h)` REASONING fact, and only when
a single rule firing already refutes `h` before any fork. (The former
full-saturation "unconditional death → `(not h)` into the parent" —
`back_prop.is_unconditional_death` / `reaches_hypothesis` — was tree-solver
machinery: dead after the tree solver's removal, deleted in S1.9.E6a.)

## Set-indexed search — monotonic engine (P1.5b S1.5b.0–.10)

The tree engine's depth-first ordering over hypothesis branches
prices in d! orderings of the same commitment set — for d=4 on
zebra2 that's 24× redundant work on each set. The **monotonic
engine** under
[`ein.py/src/ein/inference/monotonic/`](../../../ein.py/src/ein/inference/monotonic/)
collapses this by indexing by commitment **set** rather than
path: layer N enumerates every size-N alive subset via
Apriori-style prefix-join, enters each via the common
[`try_commitment_set`](../../../ein.py/src/ein/inference/commitment.py)
primitive, and merges only the unconditional consequences back
into a single root KB. There is **one entry**,
[`solve`](../../../ein.py/src/ein/inference/monotonic/solver.py): it
records every solution node (`consistent ∧ complete`, `state_hash`-deduped)
plus every refuted commitment, and
[`verdict_of`](../../../ein.py/src/ein/inference/monotonic/solver.py)
reads the verdict off the count `k` of distinct solution nodes —
`k = 0` → Contradiction (unsat core), `k = 1` → Solution, `k > 1` →
Ambiguity (gaps). These are **three answers to one problem**, selected
by the input, not by which function was called (the unsound
`gaps_solve` / `contradictions_solve` split was removed 2026-06-16 —
Q1.5b.7). The orthogonal **stop policy** (`stop_after=1` single / `N` /
`None` exhaustive) only bounds how far the lattice is walked;
`store_lattice=True` attaches a sound
[`LatticeProof`](../../../ein.py/src/ein/inference/monotonic/lattice.py)
carrying both the gaps view (`proof.solutions`) and the contradictions
view (`proof.dead_commitments` + `verdict.unsat_core`).

### Termination conditions, in order of precedence

1. **Solution at a fork.** `is_solved(result.kb, Mode.SOLVE)` on an
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
[`ProgressDumper`](../../../ein.py/src/ein/inference/monotonic/state_dump.py)
subclass streams the same events to stderr as live progress lines
(so a multi-minute exhaustive `solve` isn't a silent hang) without
needing an on-disk dump.

`MonotonicDumper` captures only the per-layer root snapshots —
on the default single-solution stop policy the engine
early-terminates, so most hypotheses are never reached and there's
nothing per-hypothesis to record. For a
**complete per-hypothesis record** — every commitment tested at
every layer, with the firings each one emitted, survivors and
casualties alike — run `solve` exhaustively (`stop_after=None`) with
a `LatticeDumper`:
see [`lattice_dump.md`](lattice_dump.md). That dump groups
`enterings/` and `kb_index/` by layer and writes
`outcome.txt` + `firings.jsonl` + `unsat_core.jsonl` per
commitment — the audit trail for debugging problem statements and
rules.

### Budget — `max_time` / `max_enterings`

`solve(..., max_time=N, max_enterings=K)` checks the
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
