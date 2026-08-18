# 06 — Saturation: the closure/boundary loop

**Settles:** the fixpoint driver, the delta discipline, and how NAF is
evaluated — §O2, §O3, §O5.
**Phase:** [P1a.3](../p1a.3_deductive_core/README.md).
**Replaces:** `ein/inference/{saturator,engine,world,contradiction,
nogoods}.py` (saturation half).

---

## 1. The algorithm, unchanged

S1.21.8 made saturation **two-phase**, and that shape is the port's
contract:

```
step():
  loop {
      if let Some(f) = closure_step() { return f }   // purely positive
      if admit_from_boundary() == 0   { return None }// one NAF admission
  }
```

- **Closure (inner).** Purely positive plans fire to quiescence. No
  negation is consulted. The `(absent …)` premises were lifted out of
  each disjunct at compile time (`split_naf` → `NafGuard`), so
  `plan.steps` is a positive program.
- **Boundary (outer).** At quiescence a `World` is built over the stalled
  KB and every parked NAF-guarded candidate is judged against that
  fixpoint. **At most one is admitted per round**, and that is a
  soundness requirement, not a throttle: admitting a batch lets one
  admission invalidate another's guard *after* its verdict was taken,
  which on `p ← absent q; q ← absent p` derives both. The port keeps
  one-at-a-time and keeps the docstring's argument with it.
- Failures **stay parked** (a `forall`'s nested `(absent (and G (absent
  B)))` can flip from failing to passing as the KB grows); an
  anti-monotone guard that fails is **retired** (`naf_retired`).

Everything else ports verbatim: priority bands (advisory since S1.21.8),
FIFO within a band via the monotone tiebreaker, `_seen` keyed on
`(binding_key, guards)` (the S1.22.0 or-disjunct fix), `naf_dropped`
structurally 0, `SaturatorStepLimitError` on `max_steps`.

---

## 2. Where the time actually goes

Exhaustive zebra2, CPython profile:

| | cum | share |
|---|---:|---:|
| `saturate` (all of it) | 19.1 s | 94 % |
| ├─ `_admit_from_boundary` | **14.7 s** | **72 %** |
| │  └─ `World.first_failing → absent → holds → _run_steps` | 11.3 s | 55 % |
| └─ `_closure_step` | 4.4 s | 22 % |
| &nbsp;&nbsp;&nbsp;├─ `_enqueue_pass` | 2.9 s | 14 % |
| &nbsp;&nbsp;&nbsp;│&nbsp;&nbsp;└─ `compile_all` | 1.7 s | 8 % |
| &nbsp;&nbsp;&nbsp;└─ `_apply` (incl. `fire`, `_record_alternative`) | 1.1 s | 6 % |

The headline is worth restating because it is not what the engine
narrative predicts: **negation costs more than deduction.** 3 178
boundary rounds issue 33 113 guard queries, and each query is a full
re-run of a negative sub-plan against the whole KB, gated only by the
`_watch_stamp` size check.

Two structural wins follow, both exact.

---

## 3. Win A — compile once, ever

`_enqueue_pass` starts with `self.engine.compile_all()`, which walks
`kb.rules.values() × _activators_for(rule)` and calls `compile_for` for
each. Measured: **253 440 `compile_for` calls, 1.45 s**, of which all but
19 are cache hits, plus 102 `Saturator.__init__` calls that each build a
fresh `Engine` and compile from scratch (0.84 s).

A `JoinPlan` is a **pure function of `(rule, activator_args)`** — that is
exactly its cache key. So ein.rs keeps a **process-wide plan memo**:

```rust
struct PlanMemo { plans: Vec<Plan>, by_key: FxHashMap<(Symbol, ArgsKey), PlanId> }
```

shared by `Arc` across every fork. A fork that derives a new activator
compiles one new plan; every other fork reuses it. Compilation happens
once per distinct pair for the whole process.

**Order caveat (important).** The *iteration order* of `Engine._cache` is
observable — `_enqueue_pass`'s full pass iterates `cache.values()`. A
fork's Python cache is built by iterating `rules` × the *fork's*
`_rule_apps_by_rule`, so a fork-derived activator for an early rule sorts
before a root activator of a later rule. A naive "share the root cache
and append" would produce a different order. So the memo holds the
*plans*; each `Engine` keeps its own `Vec<PlanId>` built in exactly
Python's order — cheap, since building it is now hash lookups, not
compiles. And the recompute is skipped entirely unless
`_rule_apps_by_rule` grew, tracked with a per-relation version counter.

Expected: ~2.5 s of 20 s (12 %) removed, exactly, with no behaviour
change — and `compile_rule` invocations drop from **17 430** to one per
distinct `(rule, activator)` pair, which on exhaustive zebra2 is ~170
(each of the 102 engines compiles the same ~170 pairs from scratch
today). The exact figure is confirmed by the `compile` events rather
than assumed.

> **Win A landed at
> [S1a.6.8](../p1a.6_performance/s1a.6.8_compile_cache_and_extents.md)
> T1a.6.8.1, 2026-08-18** (`391a506`). `plan_compile` **17 430 → 305**,
> `ein_infer::compile` **21.1 % → 2.4 %** cumulative, `solve zebra2 -e`
> −18.3 % from this half alone — 1.5× the saving this section estimated —
> and the verbose `--events` stream came out byte-identical, which is the
> order caveat below discharged rather than argued. Two corrections to the
> sketch above: the memo is `Arc<Mutex<PlanMemo>>` and lives on the
> `Session`, because `Terms` is asserted `Send + Sync` from the start for
> P1a.7's sake and an `Rc` would undo that; and it is per **run**, not
> per process, because a `PlanKey` holds `Symbol`s and a symbol only means
> something inside the `Terms` that interned it. The distinct-pair count is
> **305**, not the ~170 guessed here: forks derive activators the root never
> had. What follows is the pre-implementation record.
>
> **Win A was not implemented, and
> [S1a.6.1](../p1a.6_performance/s1a.6.1_profile_baseline.md) priced it
> (2026-08-18).** `PlanMemo` exists — as a **field of `Engine`**, so each of
> the engines a search builds still compiles every plan from scratch:
> `compile_rule` runs **17 430** times on an exhaustive `zebra2`, the number
> this section predicted, and **21.1 % of the run is inside
> `ein_infer::compile`** with 19.7 % of it under `PlanMemo::intern`
> ([baseline.md §7](../p1a.6_performance/baseline.md#7-the-top-five-costs)
> item 1). ein.py compiles exactly as many times, so this is not a parity
> defect — it is an unclaimed 21 %, which is nearly twice the 12 % this
> section expected.
>
> The order caveat below is why it is safe to claim now: the `compile` **event**
> fires on an *engine* miss and not a memo miss, and both implementations emit
> **17 250** of them on that run (identical `--events` streams, 183 231 lines
> each), so a memo underneath the engine is invisible to T2 by construction.
> [S1a.6.8](../p1a.6_performance/s1a.6.8_compile_cache_and_extents.md) landed it.

---

## 4. Win B — a semi-naive boundary

The closure is already semi-naive: after a productive firing, the next
enqueue pass processes only the derived facts (D2), seeding each
positive-premise plan *at* the new fact (D5, `run_seeded`). The boundary
is not: a parked candidate whose watch stamp moved re-runs its whole
negative query.

The `NafGuard.monotone` flag — already computed at compile time as "no
nested `(absent …)` inside this guard's sub-plan" — is the licence to fix
that:

- **Monotone guard** (the common case: a plain `(absent (P ?x))`). Its
  sub-plan is *purely positive*, so its match set only grows. If the
  guard failed to find a match at round *r* and the KB gained Δ facts by
  round *r+1*, then it finds a match at *r+1* **iff** some match uses at
  least one fact of Δ. That is precisely `run_seeded` on the guard's
  sub-plan, restricted to Δ ∩ watched-relations. Cost drops from "scan
  the extent" to "iterate the new facts".
- **Non-monotone guard** (a `forall`'s nested absent — it can flip from
  failing to *passing* as the KB grows, because adding a `B` makes the
  inner absent fail and the outer pass). Full re-evaluation, as today.
  These are a minority and the flag identifies them statically.

Three further exact refinements:

1. **Per-round guard memo.** Two parked candidates frequently share a
   guard sub-plan and a projected binding environment (`project(bindings,
   guard.scope)` collapses everything the guard does not read). Memoise
   `(guard_id, projected_env) → verdict` for the duration of one boundary
   round; the KB cannot change mid-round (at most one admission, and it
   ends the round).
2. **Version counters instead of size tuples.** `_watch_stamp` builds a
   tuple of extent *sizes* over `sorted(g.watched)` — 324 492 calls,
   0.77 s. A per-relation `u32` version bumped on append gives the same
   answer (sizes are monotone, so equal sizes ⇔ equal extents) in one
   comparison; store the last-seen version vector per parked candidate.
3. **Dirty set instead of heap churn.** Today every parked entry is
   popped and re-pushed each round. Keep the ordered structure, but
   maintain `watched_relation → parked candidates` and only *evaluate*
   the candidates whose watched relations changed — walking the parked
   set in the same priority/FIFO order, so "the first candidate whose
   guards pass" is the same candidate.

All four are verdict- and order-identical by construction; each ships
behind a T2 diff on the whole corpus.

**Not allowed:** batch admission (§1), or skipping the boundary when the
queue is non-empty (the phases are ordered for a reason).

---

## 5. Queues, delta, and the mirror

| ein.py | ein.rs |
|---|---|
| `_queue`, `_parked`: `heapq` of 6-tuples | `BinaryHeap<Reverse<(u32 priority, u64 tiebreak)>>` + a side arena of entries ([02](02_determinism_and_order.md) §2) |
| `_seen: set[(binding_key, guards)]` | `FxHashSet<(BindingKey, GuardSetId)>`; `GuardSetId` interns the per-disjunct guard tuple |
| `engine._fired: set[key]` | `FxHashSet<BindingKey>` |
| `_delta_facts: list[Fact] \| None` | `Option<SmallVec<[FactId; 8]>>`; `None` ⇒ full pass |
| `_matched_plan_ids: set[id(plan)]` | a bitset over the engine's plan-list index |
| `_pos_index: rel → [plan]`, rebuilt when the cache grows | same, rebuilt on the plan-list version |
| `_mirror_queue: list[Fact]`, consumed with `.pop()` | a `Vec<FactId>` used as a **stack** — LIFO, not FIFO, despite the name |

### `__symmetric__` native mirror

A relation marked `(__symmetric__ R)` has its extent closed under
arg-swap directly by the saturator — no plan, no matcher. Port notes:

- The cold seed iterates `_symmetric_rels()`, a **`frozenset`** — the one
  place set-iteration order leaks into firing order
  ([02](02_determinism_and_order.md) §5 H1). Confirm the hazard on
  ein.py under differing `PYTHONHASHSEED`; if it reproduces, fix ein.py
  with a `sorted(...)` first, then port the sorted version.
- The re-derivation path (mirror already exists → record an *alternative*
  justification rather than dropping it) is what makes the justification
  graph genuinely cyclic; the explanation search handles it by taking a
  least fixpoint from the sources up. Port both halves or the explanation
  minimality changes.
- Gated by `enable_symmetric_mirror`; when off, the stdlib `symmetric`
  rule covers it transparently.

---

## 5b. Win C — the fork boundary (added 2026-08-18, **not settled**)

§4 opens by saying the closure is already semi-naive and the boundary is
not. There is a third boundary neither sentence covers: **the fork**.

`try_commitment_set` forks the saturated root, writes `k ≤ 5` hypothesis
facts, and constructs a *fresh* `Saturator` — fresh engine, empty `seen` /
`fired` / `parked`, `delta = None`, which §5's table reads as a FULL pass.
So every entering full-matches every plan against a KB that is already at
a fixpoint, and re-derives the root's entire closure to discover that each
conclusion is present. The delta at that boundary is the smallest and the
best-known in the whole engine — it is the commitment set — and it is the
one place the delta is thrown away.

Measured
([P1a.6 baseline §9](../p1a.6_performance/baseline.md#9-the-fork-entry-re-derivation)):
**94.6 %** of `zebra -e`'s fork firings and **95.6 %** of `zebra2 -e`'s
are redundant, and `try_commitment_set` is 95.0 % of `zebra -e`
cumulatively. Win A's 17 430 compiles are a *symptom* of the same fresh
saturator: 12 625 of them happen inside forks, and another 4 375 in hypgen.

Unlike Wins A and B this one is **not order-identical by construction**. A
`Firing` is narrated — T2 emits one `fire` line per firing at `verbose`,
and T3 compares `--trace`'s `n_firings`, `--dump-states`' per-node
`firings` counts and the first five firings `render/shape.rs` prints. What
the fixpoint, the models, the verdict and the alternative-justification
set do is *unchanged*, and the argument for each is in
[S1a.6.9](../p1a.6_performance/s1a.6.9_fork_entry_delta.md) § What is not
at risk. So this is a decision about what the engine says it did, taken
by both implementations together or by neither:
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint).

The parity-preserving half of it is §5's own table read at the fork
boundary: the *root* beta-memories of
[05 §7](05_matcher.md) reproduce the same match sequence from a table, so
the firings still happen — they are just no longer re-discovered.

---

## 6. Firing, redundancy, alternatives

`_apply` builds every conclusion (`assert_templates` — A13 multi-assert),
looks each up, and:

- **all present** → a `redundant` Firing, no insertion, and — the
  high-volume path, ~194 k redundant firings on an exhaustive zebra2 —
  `_record_alternative` records the derivation as an alternative
  justification if any conclusion still accepts one. The O(1)
  `accepts_justification` pre-check exists to avoid building a
  `Provenance` (and stringifying bindings) on that path; ein.rs keeps the
  same shape, and gets the stringification for free by *not* doing it
  (Provenance holds `Symbol`s — [03](03_data_model.md) §7).
- **any absent** → `fire()` builds one `Provenance` shared by all
  conclusions and inserts each.

`build_fact` walks the template resolving `Reg`s; an unbound var in an
`:assert` is a hard error in ein.py (`KeyError` with a specific message)
— in ein.rs a typed error with the same rendered text.

### Contradiction detection

`contradicts(kb, fact)` — the incremental check the fail-fast fork
saturation (S1.9.E23, ~2× on exhaustive zebra2) depends on — becomes:

| shape | ein.py | ein.rs |
|---|---|---|
| `(false …)` | name compare | `rel == FALSE_SYM` |
| `(not X)` landed | `_fact_by_id(X)` — O(deg) scan | `present.test(inner_fact_id)` |
| positive `X` landed | `(rn, args) in _negated_facts` | `negated.test(fact_id)` |

Two bit tests. The full `ContradictionDetector.detect()` scan (used
pre/post fork) walks `_facts_by_relation['false']` then `['not']`;
identical in Rust, but the inner lookup is a bit test, and the result
order (direct ⊥ first, then pairs in extent order) is preserved because
it reaches the unsat core and the trace.

---

## 7. `is_stalled`

`is_stalled()` forces a fresh full enqueue pass, checks the mirror, then
checks whether any queued entry is unfired, then consults the boundary.
It is called by external drivers that wrote facts directly to the KB.
Port as-is — including the deliberate side effect of running an enqueue
pass, which changes `_tiebreaker` and therefore later ordering.

---

## 8. Acceptance for this design

- **T2 firing-sequence parity** on every fixture under
  `examples/saturation/**`, `examples/features/**`,
  `examples/domain_elim/**`, plus root saturation of `zebra.ein` /
  `zebra2.ein` (378 and 502 facts respectively — a per-fact,
  per-provenance diff).
- Counters identical: `naf_rounds`, `naf_admitted`, `naf_retired`,
  `naf_dropped == 0`, alternative-justification counts per fact.
- Win A: `compile_rule` invocation count on exhaustive zebra2 drops from
  ~17 430 to ≤ 60 (the distinct `(rule, activator)` pairs), with the
  event log proving the cache order is unchanged.
- Win B: guard sub-plan *evaluations* on exhaustive zebra2 drop by ≥ 80 %
  with an identical sequence of `park`/`admit`/`retire` events.

## Cross-links

- [05 — Matcher](05_matcher.md) — the inner loop this drives.
- [07 — Search layer](07_search_layer.md) — the caller (`try_commitment_set`).
- [08 — Parallelism](08_parallelism.md) — the enqueue pass and the
  boundary round are both read-only over the KB, which is what makes them
  parallelisable.
- [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)
  — the normative reading of `(absent …)` this loop implements.
- [`architecture_and_algorithms.md` §O2/§O3/§O5](../../../docs/kernel/inference/architecture_and_algorithms.md).
