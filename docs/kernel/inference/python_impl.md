# Inference engine — Python implementation map

The **file-by-file** developer reference for the engine. The *idiomatic*
(language-agnostic, algorithm-level) view — the nine core operations, their
CS analogs, complexity — is
[`architecture_and_algorithms.md`](architecture_and_algorithms.md); this page
is the concrete module map. Source root:
[`ein.py/src/ein/inference/`](../../../ein.py/src/ein/inference/).

> **Audience: engine contributors.** A puzzle author never reads this — the
> authoring surface is [`../ir/03-ein-lang/`](../ir/03-ein-lang/).

## Data flow

```text
KB ─▶ Engine.compile_all ─▶ JoinPlan ─▶ Saturator.saturate ─▶ reasoning facts
        (compile.py)                       (saturator.py)
                            (+ naf_guards)    │ match.py · firing.py
                                              │ at quiescence: world.py
   ┌──────────────────────────────────────────┘
   ▼ on quiescence, no goal:
 hypgen ─▶ apriori (layer N sets) ─▶ commitment.try_commitment_set ─▶ detect
                                       (fork + write + saturate)        │
                              monotonic/solver.py drives the BFS ◀──────┘
                                       └▶ verdict.py reads k → Solution / Ambiguity / Contradiction
```

## Saturation core — the deductive (monotone, append-only) layer

| module | role |
|--------|------|
| [`engine.py`](../../../ein.py/src/ein/inference/engine.py) | `Engine` driver: per-(rule, activator) compile cache; `compile_all` / `compile_for`; tracks `_fired`; `naf_dependency_map`. Its queue-less `step()` is **two-phase** (S1.21.8): purely positive matches fire first, and a NAF-guarded one is considered only at positive quiescence, against a `World` |
| [`compile.py`](../../../ein.py/src/ein/inference/compile.py) | lowers each (rule, activator) to a `JoinPlan` of opcodes: `Scan` / `Join` / `Guard` / `NestedPattern` — plus `AbsentGuard` (NAF), which `split_naf` **lifts out** of the plan (S1.21.8) so `steps` / each `extra_match_plans` disjunct is a purely positive closure plan. Each lifted guard becomes a `NafGuard` (`scope` = vars bound by the premises that preceded it, `watched` = relations the negative query reads, `monotone` = no nested absent); `JoinPlan.naf_guards` is per-disjunct, `disjuncts()` pairs them with their steps, `has_naf` reports any. A *nested* absent is not lifted — it is part of the negative query |
| [`match.py`](../../../ein.py/src/ein/inference/match.py) | runtime matcher: `_run_steps` executes a step tuple, `_bind_arg` unification, `_seed_steps` the semi-naive seed. The closure plans it runs are **purely positive** — its `AbsentGuard` arm now fires only for a guard *nested inside* another guard's sub-plan, which `run_steps` (the public step driver the boundary queries through) evaluates as one unit. `run_guarded` / `run_seeded_guarded` yield `(bindings, premises, guards)` — the match paired with **its own disjunct's** guards, which is what closes D5 structurally. `absents_still_pass` (the fire-time NAF re-check, evaluation point E2) is **deleted** |
| [`world.py`](../../../ein.py/src/ein/inference/world.py) | **the closure/world boundary** (S1.21.8) — the one place NAF is evaluated. `World(kb, commitment=())` is a read-only view of a KB *at positive quiescence*, not a snapshot: `holds(steps, bindings)` (`W ⊨ ∃x̄.·`), `absent(guard, bindings)` (`W ⊭ ∃x̄.Pθ`, run under `project(bindings, guard.scope)`), `admits` / `first_failing` over a guard tuple, and `negative_premises` — the `(relation, args)` patterns that had to fail. `root_world(kb)` is the commitment-free one |
| [`firing.py`](../../../ein.py/src/ein/inference/firing.py) | `Firing` record; `fire()` substitutes `:assert`, builds the derived `Fact` with `Provenance.from_rule` — including the S1.21.8 `absent_premises=` kwarg, the boundary queries the firing was admitted under |
| [`saturator.py`](../../../ein.py/src/ein/inference/saturator.py) | the **two-phase** fixpoint loop (S1.21.8). `_closure_step` runs purely positive plans to quiescence (priority-banded queue, delta-driven semi-naive re-enqueue, `__symmetric__` mirror); `_enqueue_binding` routes a guarded match to `_parked` instead of `_queue`; at quiescence `_admit_from_boundary` judges parked candidates against that fixpoint (a `World`) and admits **one**, then the closure re-runs. `_watch_stamp` skips re-asking a guard none of whose `watched` relations grew; a failing `monotone` guard retires its candidate. Observables: `naf_dropped` (structurally **0**), `naf_rounds`, `naf_admitted`, `naf_retired`. `_record_alternative` — the redundant-firing branch is the real dedup seam, so a re-derivation is recorded there via `kb.record_justification` (also from the `__symmetric__` mirror) |
| [`primitives.py`](../../../ein.py/src/ein/inference/primitives.py) | structural reserved atoms (`not` / `and` / `or` / `absent` / `false`) — `STRUCTURAL` |
| [`predicates.py`](../../../ein.py/src/ein/inference/predicates.py) | computed-predicate registry (`eq` / `neq`) — the `Guard` evaluators |
| [`resolve.py`](../../../ein.py/src/ein/inference/resolve.py) | leaf-node resolution in bindings |

## Hypothesis generation & commitment-lattice search — the non-monotone layer

| module | role |
|--------|------|
| [`hypgen.py`](../../../ein.py/src/ein/inference/hypgen.py) | candidate enumeration (type-blind, S1.7.23); the filter pipeline (`_negated_facts` / already-exists / lookahead / seen); `score_hypothesis`; `HypGenStats` |
| [`hrule.py`](../../../ein.py/src/ein/inference/hrule.py) | hypothesis-rule registry (`hrules` drive generation, never the saturator) |
| [`lookahead.py`](../../../ein.py/src/ein/inference/lookahead.py) | pre-branch one-step death simulator (`enable_pre_branch_lookahead`); walks `plan.disjuncts()` and evaluates each disjunct's guards in the world **with** the candidate `h` — `_guards_pass_with` asks for no match in `kb` *and* none created by `h` (the D3 fix). A guard with a nested absent is non-monotone and can't be decided that cheaply, so `_unjudgeable` skips the disjunct rather than guess — losing a kill keeps the "never reports a live hypothesis as dead" contract |
| [`apriori.py`](../../../ein.py/src/ein/inference/apriori.py) | commitment-lattice layer generation by set-size (prefix-join + no-good prune); `order_candidates` / `_set_score` — the deterministic candidate ordering |
| [`commitment.py`](../../../ein.py/src/ein/inference/commitment.py) | `try_commitment_set`: fork + write hypotheses + saturate + detect — the saturation stops at the killing firing when `enable_fail_fast_fork` (default on) |
| [`nogoods.py`](../../../ein.py/src/ein/inference/nogoods.py) | no-good learning: dead set → `root_kb._nogoods`; singletons → `_negated_facts` |
| [`monotonic/solver.py`](../../../ein.py/src/ein/inference/monotonic/solver.py) | **the main loop**: `solve()` — BFS over the commitment lattice; `_phase1_root`, `_phase2_layers`; dedup by canonical `state_key` |
| [`monotonic/lattice.py`](../../../ein.py/src/ein/inference/monotonic/lattice.py) | `LatticeProof`, `Solution`, `DeadCommitment`, `LatticeStats` |
| [`monotonic/_state.py`](../../../ein.py/src/ein/inference/monotonic/_state.py) · [`_helpers.py`](../../../ein.py/src/ein/inference/monotonic/_helpers.py) | loop state; `_compute_alive` / `_promote_forced_positives` / `_record_node` / `_handle_dead` |
| `monotonic/{state_dump,_lattice_dump,_serialise,snapshot,sanity,contract}.py` | lattice/state dumps, commutativity sanity check, the solver contract |

## Contradiction, verdict, provenance, config

| module | role |
|--------|------|
| [`contradiction.py`](../../../ein.py/src/ein/inference/contradiction.py) | detector: `(X, ¬X)` pairs (whatever either side's origin — S1.22.1b) + `(false)`; `contradicts(kb, fact)` is the O(1) incremental dual asked of each fact as it lands, which is what lets a dying fork stop saturating (S1.9.E23) |
| [`frontier.py`](../../../ein.py/src/ein/inference/frontier.py) | `smallest_contradiction_frontier` — the verdict path's unsat core; delegates the search to `explain.py`, so the answer is independent of rule-firing order (provenance-based, NAF-safe, budgeted; not a subset-minimal MUS) |
| [`explain.py`](../../../ein.py/src/ein/inference/explain.py) | minimum-cardinality explanation over the AND/OR proof graph (each fact an OR-node via [`kb.justifications`](../ir/02-data-model/02_store.md), each justification an AND-node over its `premises_raw`): ATMS-style least-fixpoint label propagation, cycle-safe by construction; `explain` / `minimal_contradiction_frontier`; `ExplanationBudget` caps the worst-case-exponential search and `Explanation.exhausted` reports truncation. Minimal over the **recorded** derivations — i.e. relative to the rule set and the saturation strategy |
| [`verdict.py`](../../../ein.py/src/ein/inference/verdict.py) | `Solution` / `Ambiguity` / `Contradiction`; verdict read from the model count `k`; `goal_bindings` |
| [`solution.py`](../../../ein.py/src/ein/inference/solution.py) | solution-node tracking; `open_hypotheses` (materialised) / `complete` (short-circuits on the generator's first element) |
| [`canon.py`](../../../ein.py/src/ein/inference/canon.py) | `state_key` — order-insensitive canonical state identity (the representation is the identity; `state_digest` is display-only) |
| [`closed.py`](../../../ein.py/src/ein/inference/closed.py) | `__closed__` handling (`CLOSED` constant; suppress guessing) |
| [`naf_deps.py`](../../../ein.py/src/ein/inference/naf_deps.py) | static NAF-dependency map; `DerivedNafWarning` — **re-grounded** by S1.21.8: no longer "this rule leans on the fire-time re-eval" (that re-eval is gone) but "NAF over a derived relation is the shape that can make a rule set non-stratified", the case where the engine reports one model of several. Advisory, `SolverConfig.warn_derived_naf` off by default; a real stratification checker is future work |
| [`why.py`](../../../ein.py/src/ein/inference/why.py) | `:why` / `:goal-text` template rendering |
| [`config.py`](../../../ein.py/src/ein/inference/config.py) | `SolverConfig` — the live solver flags (`enable_pre_branch_lookahead`, `enable_lookahead_kill_cache`, `record_alternative_justifications`, `hypgen_scoring`, `candidate_order_seed`, `lattice_order`, …) |

## Cross-cutting invariants

- **Append-only KB** — the saturator only adds facts; the one retracting flow
  is `kb.fork()` for a hypothesis branch, which takes a fresh saturator.
- **The closure is purely positive; negation happens only at the boundary**
  (S1.21.8) — every top-level `(absent …)` is lifted out of its plan at
  compile time (`compile.split_naf`), so the plans the matcher runs to
  quiescence consult no negation whatsoever, and a match says nothing about
  it. Guards are evaluated **once**, at positive quiescence, against a
  [`world.World`](../../../ein.py/src/ein/inference/world.py) — that is
  evaluation point E1, and there is no other: the fire-time re-check (E2) is
  deleted, and `Saturator.naf_dropped` is structurally 0. Admission is one
  candidate per boundary round into an empty queue, so the admitted firing
  runs against exactly the world its guard was judged against and nothing can
  go stale. Consequences: on a **stratified** rule set the result no longer
  depends on rule priority (band discipline is advisory, not load-bearing),
  and a non-stratified one is still answered by operational order — now
  boundary-admission order. Normative definition:
  [`absent_semantics.md`](absent_semantics.md); operational narrative:
  [`README.md` § NAF semantics](README.md).
- **Alive-set soundness** (the M1 invariant) — rules assert no new objects /
  relations / nested-Fact hypotheses, so `alive = f(closed KB)`; see
  [`README.md` § M1 invariant](README.md).
- **Provenance is per derivation, not per fact** — `Fact.provenance` is the
  primary justification and `kb.justifications(fact)` returns every recorded
  one, so a fact is an OR-node over AND-nodes and the proof structure is an
  AND/OR graph. The alternatives table is *history*, not a projection of
  `facts`: `rebuild_indexes()` deliberately leaves it alone, and
  `fork()` / `snapshot()` shallow-copy it per KB rather than sharing it by
  reference (a fork-local justification may name hypothesis premises root
  never assumed). Terminals take no alternatives — a `source` / `hypothesis`
  primary is the frontier, and a rule-kind primary with empty `premises_raw`
  is a synthetic engine writeback whose contract is that provenance walks
  ground out on it
  ([`reserved_engine_strings.md`](reserved_engine_strings.md)).
- **Negative dependence is recorded, not yet interpreted** (S1.21.8) — a
  firing admitted at the boundary carries the queries that had to fail in
  [`Provenance.absent_premises`](../../../ein.py/src/ein/kb/provenance.py)
  (one `(relation, args)` pattern each, `None` where the query ranged free),
  so `Deps(Y)` = `PositiveDeps(Y)` ∪ `NegativeDeps(Y)` is finally
  representable. No walk reads it yet: `kb.unsat_core`,
  [`explain.py`](../../../ein.py/src/ein/inference/explain.py) and the
  trace's "using" line still follow `premises_raw` only — which is why
  deletion-based core minimisation stays unsound (corollary C3 of
  [`absent_semantics.md`](absent_semantics.md)).

## See also

- [`architecture_and_algorithms.md`](architecture_and_algorithms.md) — the
  idiomatic (O1–O9, CS-analog) view this map is the code-level companion to.
- [`README.md`](README.md) — design principles, M1 invariant, NAF, determinism.
- [`absent_semantics.md`](absent_semantics.md) — the normative reading of
  `(absent P)` these modules implement: worlds, the single evaluation point
  E1, and the corollaries (C1–C7) the closure/boundary split rests on.
- [`reserved_engine_strings.md`](reserved_engine_strings.md) — the engine-internal
  reserved atoms these modules key on.
