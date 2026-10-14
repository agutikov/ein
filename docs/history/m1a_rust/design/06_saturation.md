# 06 — Saturation: the closure/boundary loop

**Settles:** the fixpoint driver, the delta discipline, and how NAF is
evaluated — §O2, §O3, §O5.
**Phase:** [P1a.3](../README.md#p1a3--deductive-core).

---

## 1. The algorithm, unchanged

S1.21.8 made saturation **two-phase**, and that shape is the
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
  which on `p ← absent q; q ← absent p` derives both. One-at-a-time
  stays, and so does the argument for it.
- Failures **stay parked** (a `forall`'s nested `(absent (and G (absent
  B)))` can flip from failing to passing as the KB grows); an
  anti-monotone guard that fails is **retired** (`naf_retired`).

Everything else stands as it is: priority bands (advisory since S1.21.8),
FIFO within a band via the monotone tiebreaker, `seen` keyed on
`(binding_key, guards)` (the S1.22.0 or-disjunct fix), `naf_dropped`
structurally 0, `SaturateError::StepLimit` on `max_steps`.

---

## 2. Where the time actually goes

The headline is worth stating because it is not what the engine
narrative predicts: **negation costs more than deduction.** On an
exhaustive zebra2, 3 178 boundary rounds issue 33 113 guard queries, and
each query is a full re-run of a negative sub-plan against the whole KB,
gated only by the watch stamp's size check.

Two structural wins follow, both exact.

---

## 3. Win A — compile once, ever

The enqueue pass starts with `compile_all()`, which walks
`rules × activators(rule)` and compiles each pair — nearly always a cache
hit, except that every fresh `Saturator` builds a fresh `Engine` and
compiles from scratch, 102 of them on an exhaustive zebra2.

A `JoinPlan` is a **pure function of `(rule, activator_args)`** — that is
exactly its cache key. So the engine keeps a **process-wide plan memo**:

```rust
struct PlanMemo { plans: Vec<Plan>, by_key: FxHashMap<(Symbol, ArgsKey), PlanId> }
```

shared by `Arc` across every fork. A fork that derives a new activator
compiles one new plan; every other fork reuses it. Compilation happens
once per distinct pair for the whole process.

**Order caveat (important).** The *iteration order* of an engine's plan
list is observable — the enqueue pass's full pass iterates it. A fork's
list is built by iterating `rules` × the *fork's* rule applications, so
a fork-derived activator for an early rule sorts before a root activator
of a later rule. A naive "share the root list and append" would produce
a different order. So the memo holds the *plans*; each `Engine` keeps its
own `Vec<PlanId>`, built rule by rule and activator by activator — cheap,
since building it is now hash lookups, not compiles. And the recompute
is skipped entirely unless the rule applications grew, tracked with a
per-relation version counter.

Expected: ~12 % of the run removed, exactly, with no behaviour
change — and `compile_rule` invocations drop from **17 430** to one per
distinct `(rule, activator)` pair, which on exhaustive zebra2 is ~170
(each of the 102 engines compiles the same ~170 pairs from scratch
today). The exact figure is confirmed by the `compile` events rather
than assumed.

> **Win A landed at
> [S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts)
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
> [S1a.6.1](../README.md#s1a61--fresh-profile-and-bench-baseline) priced it
> (2026-08-18).** `PlanMemo` exists — as a **field of `Engine`**, so each of
> the engines a search builds still compiles every plan from scratch:
> `compile_rule` runs **17 430** times on an exhaustive `zebra2`, the number
> this section predicted, and **21.1 % of the run is inside
> `ein_infer::compile`** with 19.7 % of it under `PlanMemo::intern`
> ([baseline.md §7](../measurements/baseline.md#7-the-top-five-costs)
> item 1) — an unclaimed 21 %, which is nearly twice the 12 % this section
> expected.
>
> The order caveat is why it is safe to claim now: the `compile` **event**
> fires on an *engine* miss and not a memo miss — **17 250** of them on that
> run — so a memo underneath the engine is invisible to the event stream by
> construction.
> [S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts) landed it.

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
2. **Version counters instead of size tuples.** A watch stamp built as a
   tuple of extent *sizes* over the sorted watched relations is 324 492
   builds on an exhaustive zebra2. A per-relation `u32` version bumped on append gives the same
   answer (sizes are monotone, so equal sizes ⇔ equal extents) in one
   comparison; store the last-seen version vector per parked candidate.
3. **Dirty set instead of heap churn.** Rather than pop and re-push every
   parked entry each round, keep the ordered structure, but
   maintain `watched_relation → parked candidates` and only *evaluate*
   the candidates whose watched relations changed — walking the parked
   set in the same priority/FIFO order, so "the first candidate whose
   guards pass" is the same candidate.

All four are verdict- and order-identical by construction; each ships
behind an unchanged event stream on the whole corpus.

**Not allowed:** batch admission (§1), or skipping the boundary when the
queue is non-empty (the phases are ordered for a reason).

### What each of them was worth — measured 2026-08-18 to 2026-08-20

All four are now closed, three of them against the measurement rather than for
it ([S1a.6.12](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot),
[baseline.md §18](../measurements/baseline.md#18-s1a612--the-boundary-and-the-premise-that-had-nothing-left-to-bind)):

| | projected | measured |
|---|---|---|
| the mechanism (monotone semi-naive) | ≥ 80 % of guard evaluations | **7.3–16.3 %** on the puzzles, **100 %** on `features/01` — [Q-M1a.17](../open_questions.md#q-m1a17--win-bs--80--assumed-monotone-guards-dominate), **declined** at a 1.4–2.2 % ceiling |
| 1. per-round guard memo | "two parked candidates frequently share a guard and env" | **0–1.2 % hit rate** — landed at S1a.3.4, **removed at T1a.6.12.2**; refinement 2 is why it hit nothing |
| 2. version counters, not size tuples | one comparison instead of a tuple | landed at S1a.3.4 as a stamp buffer on the entry, **replaced at T1a.6.12.1a by one epoch per *guard set*** — sizes are a property of the world, so they are taken once per round rather than once per candidate: 494 566 → **12 864** extent probes |
| 3. dirty set instead of heap churn | walk in the same order, evaluate only what changed | the heap churn went at S1a.3.4; the *index* was built twice at T1a.6.12.1b and **reverted twice**, at +1.5 % and +18 % |

Refinement 3 is the one worth reading twice. A per-candidate work set reaches
the ideal visit count — one per judgement, 29 865 instead of 248 043 — and is
**slower**, because a park is an ordered insert and a run parks more often than
its rounds skip. What the round actually spends is upstream of the whole
argument: `zebra -e` runs 3 216 rounds over 947 758 parked slots and visits
248 043 of them, because **a round stops at its first admission**, a quarter of
the way in. Not copying the ordered set to walk a quarter of it is worth 8 %;
knowing which quarter to walk is worth less than it costs to know.

**The lesson is about the shape of the projection, not the arithmetic.** Win B
reasoned from what a monotone guard *permits* and never asked how many parked
candidates have one — and the answer is structural: a failing monotone guard
retires its candidate immediately, so the parked set is, by construction, the
population the mechanism cannot serve. A projection over a set defined by the
complement of your own precondition is the failure mode to look for. The
section's second lesson is refinement 3's: an optimisation aimed at the loop
body when the cost is in the loop's *setup* can be exactly right about the
mechanism and still lose.

**And what the boundary's remaining cost turned out to be**, once both were
settled: not the visits and not the verdicts, but the *premises*. 85.7 % of an
exhaustive `zebra`'s candidates come from guard sub-plans, and **71.8 % of
those premises have every slot bound** by the time the walk reaches them — a
question the fact store answers with one interned lookup, where the matcher was
fetching a ten-deep participation bucket and unifying all of it
(T1a.6.12.3, −20.6 % end-to-end).

---

## 5. Queues, delta, and the mirror

| state | structure |
|---|---|
| the queue, the parked set | `BinaryHeap<Reverse<(u32 priority, u64 tiebreak)>>` + a side arena of entries ([02](02_determinism_and_order.md) §2) |
| `seen`, keyed on `(binding_key, guards)` | `FxHashSet<(BindingKey, GuardSetId)>`; `GuardSetId` interns the per-disjunct guard tuple |
| `fired` | `FxHashSet<BindingKey>` |
| the delta, or none | `Option<SmallVec<[FactId; 8]>>`; `None` ⇒ full pass |
| the matched plans | a bitset over the engine's plan-list index |
| `pos_index: rel → [plan]` | rebuilt on the plan-list version |
| the mirror queue | a `Vec<FactId>` used as a **stack** — LIFO, not FIFO, despite the name |

### `__symmetric__` native mirror

A relation marked `(__symmetric__ R)` has its extent closed under
arg-swap directly by the saturator — no plan, no matcher. Notes:

- The cold seed iterates the marked relations **sorted by name** — a
  hash-ordered set here is the one place set-iteration order would leak
  into firing order ([02](02_determinism_and_order.md) §5 H1).
- The re-derivation path (mirror already exists → record an *alternative*
  justification rather than dropping it) is what makes the justification
  graph genuinely cyclic; the explanation search handles it by taking a
  least fixpoint from the sources up. Both halves are needed, or the
  explanation minimality changes.
- Gated by `enable_symmetric_mirror`; when off, the stdlib `symmetric`
  rule covers it transparently.

---

## 5b. Win C — the fork boundary (added 2026-08-18, **landed 2026-08-19**)

> **Settled.** [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
> was answered **yes**: `Saturator::resume` is the shipping path, and the
> fresh saturator stays behind the `fork-delta` feature (`EIN_FORK_DELTA=0`)
> as the control arm. Fork firings −74 % / −77 %, fork compiles → 0,
> `solve zebra.ein -e` 525.6 → 394.2 ms — the milestone's last unmet target.
> What decided it is that the **answers** are identical — every verdict, model, unsat core and
> `summary.json` counter, over 3.2 M enterings — while the narration is not.
> One prediction was wrong: the alternative-justification set does *not*
> survive. 267 529 facts record a different, equally valid derivation first,
> because a resumed fork inherits root's parked candidates with root's
> tiebreakers and the boundary admits one per round.
> [baseline.md §11](../measurements/baseline.md#11-the-resumed-fork-saturator-measured)
> has all of it.


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
([P1a.6 baseline §9](../measurements/baseline.md#9-the-fork-entry-re-derivation)):
**94.6 %** of `zebra -e`'s fork firings and **95.6 %** of `zebra2 -e`'s
are redundant, and `try_commitment_set` is 95.0 % of `zebra -e`
cumulatively. Win A's 17 430 compiles are a *symptom* of the same fresh
saturator: 12 625 of them happen inside forks, and another 4 375 in hypgen.

Unlike Wins A and B this one is **not order-identical by construction**. A
`Firing` is narrated — the event stream carries one `fire` line per
firing at `verbose`, and `--trace`'s `n_firings`, `--dump-states`' per-node
`firings` counts and the first five firings `render/shape.rs` prints all
show it. What
the fixpoint, the models, the verdict and the alternative-justification
set do is *unchanged*, and the argument for each is in
[S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) § What is not
at risk. So this is a decision about what the engine says it did:
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint).

The narration-preserving half of it is §5's own table read at the fork
boundary: the *root* beta-memories of
[05 §7](05_matcher.md) reproduce the same match sequence from a table, so
the firings still happen — they are just no longer re-discovered.

---

## 6. Firing, redundancy, alternatives

Applying a match builds every conclusion (`assert_templates` — A13
multi-assert), looks each up, and:

- **all present** → a `redundant` Firing, no insertion, and — the
  high-volume path, ~194 k redundant firings on an exhaustive zebra2 —
  the derivation is recorded as an alternative justification if any
  conclusion still accepts one. The O(1) `accepts_justification`
  pre-check exists to avoid building a `Provenance` on that path, and
  bindings are never stringified at all (Provenance holds `Symbol`s —
  [03](03_data_model.md) §7).
- **any absent** → `fire()` builds one `Provenance` shared by all
  conclusions and inserts each.

`build_fact` walks the template resolving `Reg`s; an unbound var in an
`:assert` is a hard error — a typed error, rendered as a `KeyError` line
([defined_behaviour.md](../../../kernel/defined_behaviour.md#4-errors-and-exit-codes)).

### Contradiction detection

`contradicts(kb, fact)` — the incremental check the fail-fast fork
saturation (S1.9.E23) depends on — is:

| shape | check |
|---|---|
| `(false …)` | `rel == FALSE_SYM` |
| `(not X)` landed | `present.test(inner_fact_id)` |
| positive `X` landed | `negated.test(fact_id)` |

Two bit tests. The full detection scan (used pre/post fork) walks the
`false` extent then the `not` extent; the inner lookup is a bit test, and
the result order (direct ⊥ first, then pairs in extent order) is fixed
because it reaches the unsat core and the trace.

---

## 7. `is_stalled`

`is_stalled()` forces a fresh full enqueue pass, checks the mirror, then
checks whether any queued entry is unfired, then consults the boundary.
It is called by external drivers that wrote facts directly to the KB.
That includes the deliberate side effect of running an enqueue pass,
which advances the tiebreaker and therefore changes later ordering.

---

## 8. Acceptance for this design

- `naf_dropped == 0` on every corpus entry.
- Win A: `compile_rule` invocation count on exhaustive zebra2 drops from
  ~17 430 to ≤ 60 (the distinct `(rule, activator)` pairs), with the
  event log proving the cache order is unchanged.
- ~~Win B: guard sub-plan *evaluations* on exhaustive zebra2 drop by ≥ 80 %
  with an identical sequence of `park`/`admit`/`retire` events.~~ **Retired
  2026-08-19**: only **7.3 %** of an exhaustive zebra2's evaluations are on a
  guard the mechanism can help ([Q-M1a.17](../open_questions.md#q-m1a17--win-bs--80--assumed-monotone-guards-dominate)),
  so this clause measures a lever that is not there. The identical-sequence half
  of it stands and is unconditional. The boundary's *speed*
  acceptance moves to
  [S1a.6.12](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot)'s, stated
  in cumulative profile share rather than in evaluations, because after the
  refinement that never landed the evaluation count is no longer what the
  boundary spends its time on.

## Cross-links

- [05 — Matcher](05_matcher.md) — the inner loop this drives.
- [07 — Search layer](07_search_layer.md) — the caller (`try_commitment_set`).
- [08 — Parallelism](08_parallelism.md) — the enqueue pass and the
  boundary round are both read-only over the KB, which is what makes them
  parallelisable.
- [`absent_semantics.md`](../../../kernel/inference/absent_semantics.md)
  — the normative reading of `(absent …)` this loop implements.
- [`architecture_and_algorithms.md` §O2/§O3/§O5](../../../kernel/inference/architecture_and_algorithms.md).
