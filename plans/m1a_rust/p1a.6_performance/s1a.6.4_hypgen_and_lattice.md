# S1a.6.4 — Hypgen and lattice hot paths

**Phase:** P1a.6 (Performance)
**Estimate:** 3 days
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md)
**Implements:** refinements to
[design/07](../design/07_search_layer.md) §§2, 4

**Status: shipped 2026-08-19, and the cost was not where the stage was
aimed.** The tasks were written against "~18 k raw candidates per call"; a
call offers **125**. What it *does* spend is the setup it repeats per call and
a blind enumerator that **no milestone workload reaches** — zebra and zebra2
both declare an `(hrule …)`, so `generate` returns before
`candidate_objects`, and the corpus's slowest solves are the ones that do not.

`solve examples/features/05_stdlib_domain_elim.ein -e` **4182.1 → 3559.7 ms
(−14.9 %)**, `features/01 -e` **−15.1 %**, `branching/07 -e` −9.5 %,
`sq-bwd/houses -e` −4.2 %. On the four targets: `solve zebra2 -e` 42.7 →
**41.7 ms**, `solve zebra -e` 78.1 → **77.1 ms**, `solve zebra2` −5.5 %,
`saturate zebra2` −7 %; `saturate_root/zebra2` and `boundary/zebra2`
**−16 %** each. Every work counter identical to the digit, T3 472/473 and T2
239/240 with [D2](../divergences.md) the only cell, and the whole repo gate
green: **1 506 pytest + 21 acceptance + 305 ein.rs**, every golden unchanged.

| task | outcome |
|---|---|
| **T1a.6.4.0** (new) the key the engine walk builds | **shipped** — a symbol activator argument is already its own key; `complete()` 61.5 → 47.0 µs on zebra2 |
| **T1a.6.4.0b** (new) the walk itself | **shipped** — no `Rule` clone, one activator buffer, two dead clones removed; setup 31.0 → 22.2 µs, `boundary/zebra2` −16.9 % |
| [T1a.6.4.1](#task-t1a641--intern-on-probe) intern-on-probe | **not built** — the premise is ein.py's; here probe *is* intern, and `FactStore::intern` is 0.69 % of the heaviest blind run |
| [T1a.6.4.2](#task-t1a642--hoisted-candidate-object-list) hoisted candidate list | **shipped, re-aimed** — `candidate_objects` **10.7 % → 3.1 %** of `features/05 -e` |
| [T1a.6.4.3](#task-t1a643--relationslot-precomputation) relation/slot precomputation | **shipped** — plus `by_participation`'s key; `branching/07 -e` −7.4 % |
| [T1a.6.4.4](#task-t1a644--no-good-bitmask) no-good bitmask | **not built, measured in its own regime** — the no-good machinery is **0.3 %** of the run design/07 §4 predicted it would dominate |
| [T1a.6.4.5](#task-t1a645--incremental-alive-maintenance) incremental alive | **not built** — the alive set is recomputed **6 times** per solve, not per entering |
| [T1a.6.4.6](#task-t1a646--complete-fast-path) `complete()` fast path | **recorded as wrong, and now unnecessary** — the setup it would have raced was 71 % of the call and is gone |

The numbers are
[baseline.md §15](baseline.md#15-s1a64--the-per-call-setup-and-the-enumerator-the-targets-never-run).

## Context

The search layer was ported for fidelity, not speed: hypgen still
enumerates ~18 k candidates per full call, `complete()` still asks the
generator for its first element, and the alive set is still recomputed
from scratch per layer. All of that is correct and all of it is
constant-factor work that integers make cheap — but only if the
allocations go away too.

The constraint that shapes every task here: **`HypGenStats` attribution
is a T1 observable**, so nothing may change which filter drops a
candidate, or in what order.

**Re-aimed by its own measurement (2026-08-19).** Three of those sentences
are about ein.py and the fourth is about a workload the phase does not run:

- **~18 k candidates per call is ein.py's blind enumerator on zebra2.** ein.rs
  offers **125** raw candidates per pass on zebra and zebra2 (336 on
  `terminus`, 120 on `features/05`), because both puzzles are hrule-driven.
- **The call count is small and the per-call cost is not.** An exhaustive
  `zebra2` makes **36** generation passes — 34 of them `complete()` — and
  builds **7 884 compile-cache keys** doing it: 219 per call, against those
  125 candidates. The new `plan_key`, `hypgen_call`, `hypgen_complete` and
  `lookahead_probe` counters are how that was found.
- **`_compute_alive` runs 6 times a solve**, not once per layer-that-matters:
  `hypgen_call − hypgen_complete` is 6 on every workload measured, including
  the one with 384 167 enterings.
- **The workloads that exercise this stage's subject are not in the target
  set.** `features/05_stdlib_domain_elim -e` is 4.2 s where `solve zebra -e`
  is 78 ms, and 26.6 % of it was `hypgen/branch` where `zebra -e` is 7.9 %.
  `utils/e2e_baseline.py --blind` is the row set that fixes this.

## Acceptance

- T1 identical: every `HypGenStats` key, every entering counter. **Met** —
  every counter on all four milestone cells is identical to the digit, and
  T2/T3 are 239/240 and 472/473 with D2 the only cell.
- T2 identical on the branching fixtures. **Met.**
- Measured: candidate-enumeration cost per `complete()` call and per
  `open_hypotheses()` call, before and after. **Met** —
  `examples/hypgen_calls.rs` is the instrument, and it exists because
  `hypgen_cost`'s 100-round mean is dominated by round 1, the one whose
  lookahead probes fill the kill cache.
- Zero allocations for a rejected candidate. **Already true, and checked** —
  the row key is the identity, so a rejected candidate costs one intern
  (a hash lookup) and two bit tests, and nothing on that path allocates.

## Tasks

### Task T1a.6.4.0 — The key the engine walk builds

> **Shipped.** `plan_key` is `(rule_name, tuple(str(a) for a in
> activator.args))` and built it the way ein.py must: render each argument to
> a `String`, intern the text back to a `Symbol`. For a **symbol** argument —
> every activator in the corpus — that round-trip is the identity, because
> `Terms::display` of a `Tag::Sym` value *is* that symbol's text. It now takes
> the symbol and renders only an `int` or a nested fact, which keeps the
> deliberate `7` / `'7'` collision the key is documented to have.
> `complete()` 61.5 → **47.0 µs**, setup 43.4 → **31.0 µs**;
> `plan_key_renders_only_what_needs_rendering` checks both routes against the
> pre-shortcut body over five activator shapes and asserts it reached both
> branches, verified by mutating each in turn.

### Task T1a.6.4.0b — The walk the setup repeats

> **Shipped.** `Engine::compile_all` is what a fresh `Lookahead` runs per call
> — ~120 `(rule, activator)` pairs, ~40 times a solve — and it allocated three
> ways before compiling anything: a `Vec<Rule>` of *cloned* rules, a
> `Vec<Option<FactId>>` per rule, and `activator_args` + `reg_names` clones per
> engine miss for a `check_layout` that is a debug-build assertion. None was
> needed. Setup 31.0 → **22.2 µs**, and — unplanned — `saturate_root/zebra2`
> **−16.2 %** and `boundary/zebra2` **−16.9 %**, because root saturation runs
> the same walk per enqueue pass and its cost scales with the rule count.

### Task T1a.6.4.1 — Intern-on-probe

> **Not built: the premise is ein.py's.** ein.py builds a `Fact`, a
> `Provenance` slot and two tuple hashes per raw candidate; here the *row key
> is the identity*, and `FactStore::intern` is a probe plus a push on a miss —
> so "probe, and only materialise on survival" is the same hash lookup with a
> branch added and a second lookup for the caller. Measured: `FactStore::intern`
> is **0.69 %** of `features/05 -e` (the heaviest blind-mode run) and 0.39 % of
> `zebra2 -e`, over 125–336 raw candidates per pass. The `seen_in_call` dedup
> the task worries about is already an `FxHashSet<FactId>` over the interned
> id — the open-addressed row-key table, by another name.

Compute a candidate's row key and run the two bit tests
(`negated_fact`, `fact_already_exists`) *before* interning or
materialising anything. Only a candidate that survives them becomes a
`FactId`. This is the difference between 18 k interns and ~100 per call.

Care: the `seen_in_call` dedup also wants a key. Use the row key's hash
plus an equality check against a small open-addressed table, rather than
interning to get a `FactId` to dedup on.

### Task T1a.6.4.2 — Hoisted candidate-object list

> **Shipped, and it is the stage's largest win.** The hoist itself was already
> done (T1a.4.1.1 — `candidate_objects` runs once per `generate`, not once per
> `(object, relation, slot)`), so what was left was the *build*, and on a
> blind-mode puzzle that build is **10.7 %** of the run:
>
> - the type-role set was an `FxHashSet` filled from every relation signature
>   per call → a `BitSet`, one OR per signature entry, because a `Symbol` is a
>   dense `u32`;
> - `Kb::names`' dedup was a hash set and it walks **every layer**, so a fork
>   20 deep paid 20 layers of hashing per pass → the same bitset argument;
> - the sort **drops first and sorts the survivors** — they commute, a rank
>   being a distinct `u32` per symbol — and reads `Interner::ranks` once rather
>   than re-entering its `OnceCell` per comparison, which was 70 % of that
>   symbol's samples.
>
> `candidate_objects` **10.7 % → 3.1 %**; `features/05 -e` −7.6 %,
> `features/01 -e` −7.8 %.

Land the hoist from
[S1a.4.1](../p1a.4_search_layer/s1a.4.1_hypothesis_generation.md)
T1a.4.1.1 if it was deferred, and cache the derived sets — the type-role
atoms (from relation signatures) and the reserved names — on the
`Program`, since they cannot change after load.

**Not** cached on the `Program`: the sets are rebuilt per call and made cheap
instead. Caching needs a validity key, and the honest one is per-KB — the name
set grows with the facts a fork derives — which is the same question the
`Lookahead` engine raises below and neither is worth a wrong answer.

### Task T1a.6.4.3 — Relation/slot precomputation

> **Shipped, and it found a second loop of the same shape.** `raw_candidates`
> rebuilt the `(relation, arity)` list and re-decided all three pre-candidate
> skips for **every focal object**, and `is_closed` walked the whole
> `(__closed__ …)` extent to do it — a question asked `|objects| × |relations|`
> times a pass whose answer cannot change during one, since the only KB write a
> pass makes is the kill cache's `(not h)` and its head is `not`.
> `relation_plan` answers it once, in the same order; the skip **counters** and
> the `hypskip` events stay per (focal, relation) where the observables want
> them, which is the task's own "must preserve the pre-candidate counter bumps
> exactly".
>
> And `by_participation`: `sort_by_key` evaluates its key on every comparison,
> and `Kb::participation` sums a name's entry across the whole layer stack.
> Decorate, sort, undecorate — one walk per name.
>
> `branching/07 -e` **−7.4 %**, the cell whose lookahead is off and therefore
> has no per-candidate cost to hide per-focal work behind.

The `(relation, slot)` enumeration order and the per-relation skip
verdicts (`closed`, whitelist, blacklist) are recomputable only when the
`(__closed__ R)` extent or the query changes. Cache them with a version
counter, rebuilt on change. Must preserve the *pre-candidate counter*
bumps exactly — a skip that is cached still has to be counted.

**No version counter.** Per *call* is exact and needs no invalidation
argument; per *run* would need one, and the extent it reads is small.

### Task T1a.6.4.4 — No-good bitmask

> **Not built, and measured in the regime that was supposed to justify it.**
> [design/07 §4](../design/07_search_layer.md) says the ≤ 64-alive `u64` mask
> "matters more than zebra2 suggests: with `enable_singleton_writeback` off,
> the exhaustive search explodes to 3 336+ enterings and a correspondingly
> large clause set, and that is the regime where clause checking dominates".
> The explosion is real — **3 831 enterings, 354 clauses, 2.38 s** — and in
> that run the whole apriori/no-good machinery is **0.3 %** of it:
> `filter_candidate` 0.3 %, `nogood` and `is_subset` 0.0 %, the
> `contradiction` bucket 0.1 % self. `admit_from_boundary` is **60.2 %**.
>
> The clause store is subsumption-minimal on emit, which is what keeps 3 831
> dead enterings down to 354 clauses; the subset test the mask would replace is
> a merge over sorted `u32` slices that runs at most 354 times per candidate
> and stops early. A second representation would be a second thing to keep
> right for an instruction that is not being executed —
> [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)'s
> lesson, which `nogoods.rs` cites as its reason for carrying the trigger here
> instead of building it at S1a.4.3.

Land the ≤64-alive bitmask representation from
[S1a.4.3](../p1a.4_search_layer/s1a.4.3_apriori_and_nogoods.md) if it
was deferred, and measure it in the regime where it matters:
`enable_singleton_writeback=false` on zebra2, where the search explodes
to 3 336+ enterings and the clause set grows with it.

### Task T1a.6.4.5 — Incremental alive maintenance

> **Not built, by the task's own gate.** It says do this "**only** if the
> profile still shows `_compute_alive` after the tasks above". The profile
> shows the `alive/closed` bucket at 0.5–2.6 % self, most of it `solve.rs`'s
> own loop, and the counters show why: `open_hypotheses` runs **6 times** in
> an exhaustive `zebra2`, an exhaustive `zebra`, `features/05 -e` (384 167
> enterings) and `branching/07 -e` alike. It is per *layer*, and layers are
> single digits. The riskiest task in the stage would have bought six passes.

`_compute_alive` re-runs the whole generator per layer. The alive set
changes only by: a candidate becoming a fact, a `(not h)` writeback, or a
forced-positive promotion — all of which are single-fact events the
engine already knows about. Maintaining the set incrementally is
plausible, but it is the riskiest task here: the generator's output is
defined by the *pipeline*, not by a set difference, and the lookahead
filter is not monotone in an obvious way.

Do it **only** if the profile still shows `_compute_alive` after the
tasks above, and gate it behind a debug assertion that recomputes from
scratch and compares, run in every conformance build.

### Task T1a.6.4.6 — `complete()` fast path

`complete()` needs only *one* surviving candidate. Order the enumeration
so the cheapest-to-survive candidates come first — **no**: that changes
which candidate is found, and `complete` is a boolean, so it would not
change the answer... but it *would* change the kill-cache writes the
lookahead makes along the way, which are root-visible facts. So: leave
the order alone, and make the per-candidate cost lower instead. Recorded
here because it is a tempting change that is wrong.

> **Still wrong, and now also unnecessary.** The measurement the note never
> had: a `complete()` call on zebra2 was **61.5 µs of which 43.4 µs was
> setup** — a call that short-circuits on candidate #1 paid for a whole fresh
> `Engine` before looking at it. Racing to the first survivor would have
> optimised the 18 µs and left the 43. It is 38.4 µs now, of which 23.0 is
> setup, and what remains of *that* wants a per-KB cache rather than an
> ordering change (see below).

## What was measured and deliberately left

**The `Lookahead`'s engine is still rebuilt per call** — 22 µs of a 38 µs
`complete()` on zebra2, ~1 % of `zebra2 -e`. Removing it needs the plan list
to be cached, and the cache key has to be exact: the walk is `rules ×
activators`, forks derive activators root never had (305 memo entries against
design/06's predicted ~170; 17 540 in the no-writeback regime), and a plan set
that is a *superset* of the KB's can produce a lookahead kill the oracle does
not — a `HypGenStats` difference, which is T1. A per-KB-lineage cache keyed on
`n_rule_apps` is the shape that would work, which is the same shape
`Engine::compile_all`'s own shortcut already uses; it wants the KB to own the
cache, and `Plan` lives a crate above `Kb`. Left, with the number.

**The `compile` events would not have stopped it.** Since
[S1a.6.10](s1a.6.10_parity_contract.md) `compile` is on the parity contract's
elided list — counted and reported, not compared — so the ~4 600 per
exhaustive `zebra2` that this walk emits are narration by the contract's own
rule. That was checked before the work was scoped, not after.

## Notes

- The profile's `hypgen/branch` row in `utils/profile_solve.py` is
  inflated by nested `saturate` calls; use the Rust profile's own
  attribution, not that rollup. **Confirmed, and it cuts the other way too**:
  the Rust bucket's 7.9 % on `zebra -e` is mostly the allocator under
  `try_commitment_set`, while `ein_infer::hypgen` frames are 0.04 % of the
  leaves. `--cum-of hypgen::generate` is the number that means what the row
  name says: **2.1 %**.
- If hypgen does not appear in [S1a.6.1](s1a.6.1_profile_baseline.md)'s
  top five, skip tasks 3–5 and say so in the stage log. **It does not, on the
  milestone workloads** — and task 3 was run anyway, because the blind-mode
  cells the top five never covered are where it lives. 4 and 5 are skipped
  against numbers above, which is the same rule applied with a measurement
  instead of a rank.
