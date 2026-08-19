# S1a.6.12 — The NAF boundary and the per-entering snapshot

**Phase:** P1a.6 (Performance)
**Status:** **shipped 2026-08-20** — five tasks, four landed, one declined
with its number, and one of the four aimed somewhere the stage did not
predict. `solve zebra -e` **76.7 → 47.5 ms**, `zebra2 -e` 41.1 → 28.9,
`features/05 -e` 3673 → 3010, and **no cell in either bench set slower**.
The measurements are [baseline.md §18](baseline.md#18-s1a612--the-boundary-and-the-premise-that-had-nothing-left-to-bind);
what each task did is at the end of its section below.
**Estimate:** 4 days
**Depends on:** [S1a.6.3](s1a.6.3_beta_memories.md) (which named it),
[S1a.6.9](s1a.6.9_fork_entry_delta.md) (whose snapshot this halves the cost of)
**Implements:** [design/06](../design/06_saturation.md) § Win B, refinement 3 —
and **declines** its headline mechanism, with the number
**Chosen by:** [baseline.md §17](baseline.md#17-the-boundary-measured-before-the-stage-that-aims-at-it)

## Why this, and why now

Every stage since [S1a.6.3](s1a.6.3_beta_memories.md) has ended by naming the
same two blocks, and each one has grown as a *share* while shrinking in
absolute terms, because everything around them got faster:

| cone | `zebra -e` | `features/05 -e` (blind) |
|---|---:|---:|
| `Saturator::admit_from_boundary` | **37.7 %** cumulative, 7.5 % self | **28.2 %**, 4.4 % self |
| ↳ of which `Matcher::holds` — the guard queries | 22.2 % | 18.0 % |
| ↳ the rest — *visiting* parked candidates at all | ~15.5 % | ~10.2 % |
| `Saturator::resume` — the per-entering snapshot | **10.3 %** | **12.4 %** |
| ↳ `Vec::clone⟨Entry⟩` alone | 3.5 % self | 3.2 % self |

They are the only two blocks left above 10 % on *both* shapes of workload, and
the split inside the boundary is the finding this stage is built on: **a third
of it is not the queries.** It is walking an ordered set of parked candidates
and building a watch stamp for each, once per round, to discover that almost
none of them can have changed.

**Nothing here is new work the port invented.** design/06 § Win B specified
four refinements; two landed at [S1a.3.4](../p1a.3_deductive_core/s1a.3.4_world_and_contradiction.md)
and were worth ~2 % at root scale, one is this stage's T1a.6.12.1, and the
headline mechanism — semi-naive re-evaluation of *monotone* guards — has been
waiting on [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
since it was found to reach a tenth of its stated ≥ 80 %.

**Q-M1a.17's open half is now measured, and it closes the question.** It asked
whether the *exhaustive* mix differs from the root-scale one. It does — in the
wrong direction:

| cell | guard evaluations | of which monotone |
|---|---:|---:|
| `zebra2` root (S1a.3.4) | 958 | 109 (11.4 %) |
| `zebra` root (S1a.3.4) | 945 | 280 (29.6 %) |
| `solve zebra2 -e` | 30 691 | **2 250 (7.3 %)** |
| `solve zebra -e` | 29 505 | **4 505 (15.3 %)** |
| `solve features/05 -e` | 4 719 834 | **493 985 (10.5 %)** |

The structural reason Q-M1a.17 gave is exactly why: a candidate that is *still
parked* has a guard that failed, a failing **monotone** guard retires its
candidate on the spot (`admit_from_boundary`'s `Some(g) if …monotone` arm), and
so every re-judged candidate is one whose failing guard is a `forall`'s nested
absent — which design/06 excludes from the mechanism by name. Scale makes the
retirement *more* effective, not less. So Win B's ≥ 80 % is 7–15 %, its ceiling
on `zebra -e` is 15.3 % × 22.2 % = **3.4 % end-to-end**, and this stage spends
its days elsewhere.

## Targets

The four milestone targets are met with room and this stage is not aimed at
them; it is aimed at the two blocks above, on **both** workload shapes.

- `admit_from_boundary` cumulative **below 28 %** on `zebra -e` and below 22 %
  on `features/05 -e`, or a written account of why not.
- `Saturator::resume` cumulative **below 6 %** on both.
- End-to-end: ≥ 8 % on `solve zebra -e`, ≥ 5 % on `features/05 -e`, and no cell
  in `utils/e2e_baseline.py --blind` slower.

## The rail this stage runs on

`naf_rounds`, `naf_admitted` and `naf_retired` are in `summary.json`: **T1
observables, and none of them may move.** So may the `park` / `retire` /
`admit` event stream not move, which T2 compares.

That is a tighter constraint than it looks, and it is also what makes the
stage's central optimisation *provable* rather than merely tested. A round
today does this per parked candidate: build the watch stamp, compare it against
the one stored at that candidate's last failed judgement, and skip if equal —
where the stamp is the extent sizes of every relation its guards read, and the
KB only grows, so equal sizes mean equal extents mean an unchanged verdict.
**Every task below preserves that predicate exactly**; what they change is how
much work it takes to evaluate it, not what it answers. A candidate skipped by
a cheaper test is a candidate that today's stamp also skips, so it emits what
it emits today: nothing.

## Tasks

### Task T1a.6.12.1 — Visit what changed, not everything

**The measurement.** `watch_stamp` counts one stamp build per parked candidate
per round, `guard_query` counts guards actually asked:

| cell | visits (`watch_stamp`) | extent probes (`watch_stamp_rel`) | guards asked | visits per ask |
|---|---:|---:|---:|---:|
| `solve zebra2 -e` | 204 158 | 406 106 | 30 691 | **6.7** |
| `solve zebra -e` | 248 043 | 494 566 | 29 865 | **8.3** |
| `solve zebra2` | 36 943 | 73 427 | 9 978 | 3.7 |
| `solve zebra` | 41 040 | 81 768 | 8 827 | 4.7 |
| `solve features/05 -e` | 4 755 421 | 8 981 278 | 4 755 413 | **1.0** |

At most 12 % of `zebra -e`'s visits reach a query (`guard_query` is an upper
bound on candidates judged, since a candidate may ask several guards) — so **at
least 88 % of the walk exists to discover that nothing changed**, and pays two
extent probes per watched relation to discover it. `btree::map::Iter::next` is
3.2 % of self time on its own, 95 % of it under this loop.

**The change**, and it is design/06 § Win B refinement 3, the one that did not
land: maintain `watched relation → parked candidates`, and a per-round set of
relations that grew. A round then walks the *affected* candidates in the same
`(priority, tiebreaker)` order, instead of walking all of them and asking each
one. Unjudged candidates are always affected (their stamp means nothing yet).

**Why it is order-identical.** Admission ends a round, so the only ordering
question is which candidate is admitted first — and a candidate whose watched
relations did not grow cannot be admitted, because its guards' verdicts are
unchanged and one of them failed last round. Skipping it therefore cannot
reorder anything. This is the same argument the stamp already rests on, applied
one step earlier.

**Gate.** `watch_stamp` down ≥ 5× on both puzzles; `naf_rounds`,
`naf_admitted`, `naf_retired`, every T1 counter and the T2 event stream
unchanged on the whole corpus; ≥ 5 % end-to-end on `zebra -e`.

`features/05` is the control that says what this does *not* buy: 1.0 visits per
ask, because its 384 167 forks each judge a small parked set once. The task is
worth what it is worth on the deep saturations, and the blind cells are where
T1a.6.12.5 pays instead.

**Outcome — landed in two halves, and the index was not one of them.**

*T1a.6.12.1a: the stamp is a property of the world, not of the candidate.*
Sizes are taken **once per round** and every guard set whose relations grew
gets that round's epoch; a candidate is stale exactly when its set's epoch is
past the epoch it was judged at. The same predicate in two integer loads, and
the same predicate *exactly* — the KB cannot change while the boundary runs, so
every judgement sees the sizes its own round recorded. Boundary extent probes
**494 566 → 12 864** on `zebra -e` (38×), 406 106 → 8 805 on `zebra2 -e`;
`zebra -e` −7.2 %. Its side effect is the one T1a.6.12.5 needed: `judged_at`
moved to a side table and `Entry` became written-once-at-enqueue.

*T1a.6.12.1b: the index was built twice and reverted twice.* A per-candidate
ordered work set reaches the ideal visit count — 29 865, one per judgement —
and costs **+1.5 % instructions**, because a park is an ordered insert and
there are more parks than the visits they save. Per-guard-set chains cost
**+18 %**. Then the instrumentation nobody had taken said what the round
actually spends: **3 216 rounds, 947 758 parked slots copied, 248 043
visited**, because a round stops at its first admission a quarter of the way
in. The cost was never the visits — it was copying the ordered set to walk a
quarter of it. The walk now holds the set and defers its removals, and the
`fired` probe (a `BindingKey` hash per candidate per round, **3.8 % of self
time** for a branch no corpus program takes) is gated on `may_collide`.
`zebra -e` **−8.1 %**, and `examples/features/08_disjunct_guard_sets.ein` is
the fixture that takes the branch.

**The gate, honestly:** `watch_stamp` is **unchanged at 248 043**. The ≥ 5× it
asked for was written against a model where a visit costs two extent probes and
a vector compare; T1a.6.12.1a made a visit two integer loads, and then the
measurement said the ordered-set copy was the cost. Both work sets that would
have moved the counter were built, measured and reverted — [Rule 3](README.md#rules-for-this-phase),
with numbers.

### Task T1a.6.12.2 — The per-round guard memo, priced

**The measurement.** `guard_query − guard_eval` is the memo's hit count:

| cell | guards asked | queries run | memo hits |
|---|---:|---:|---:|
| `solve zebra2 -e` | 30 691 | 30 691 | **0 (0.0 %)** |
| `solve zebra -e` | 29 865 | 29 505 | 360 (1.2 %) |
| `solve features/05 -e` | 4 755 413 | 4 719 834 | 35 579 (0.75 %) |

Win B's first refinement assumes "two parked candidates frequently share a
guard sub-plan and a projected binding environment". They do not — because the
watch stamp filtered them out before they could. The two refinements overlap,
and the cheaper one wins every time. What is left is the cost: a
`Box<[Value]>` allocated for the key and a hash insert **per evaluation** —
4.7 M of them on `features/05 -e`, on the hottest path in that run.

**The change:** remove it, or make the key allocation-free if removal
regresses. It is a pure function cache over a KB that cannot change mid-round,
so nothing observable can move either way — which makes this the stage's
cheapest measurement and the one to run *first*, since T1a.6.12.1 will only
lower the hit rate further.

**Gate:** no observable moves (guaranteed by construction, verified by T2), and
`features/05 -e` improves. If it does not, keep the memo and record the number
next to design/06 § Win B refinement 1.

**Outcome — removed, on the work rather than the clock.** Instructions retired
**44.29 → 42.41 G** on `features/05 -e` (−4.3 %) and 1.0197 → 1.0049 G on
`zebra -e`, with 4.7 M allocations gone; wall time unchanged either way. The
control is what settles it: `branching/07 -e` has **zero guards**, executes the
same 14.35 G instructions in both builds, and still differs by 1.3 % in cycles —
this machine's layout noise is larger than the memo ever was. `guard_eval` now
equals `guard_query` by construction, and the pair is kept so a memo proposed
here again has to earn the gap back.

### Task T1a.6.12.3 — What the guard queries scan

`Matcher::holds` is 22.2 % of `zebra -e` and 18.0 % of `features/05 -e`, and
after T1a.6.12.1 it is nearly all of what the boundary costs. On `zebra -e` the
two `Kb::facts_with` iterator adapters are **10.8 % of self time**, which is an
extent scan.

[S1a.6.3](s1a.6.3_beta_memories.md)'s T1a.6.3.0 made the *join* 4.5× faster by
keying the index one level inside a nested argument, and guard sub-plans go
through the same `Matcher::walk` driver — so they should already benefit. This
task is to find out whether they do: split `scan_bucket` / `scan_extent` and
`cand_bucket` / `cand_extent` by caller (join vs guard), which today are
whole-run totals, and then decide against the split rather than against a
guess. A guard whose premise offers no bound argument to key on will scan
whatever it scans; a guard that *could* use the index and does not is a bug
with a 10 % price tag.

**Gate:** the instrument lands regardless (it is four counter fields); the
optimisation only if the split says there is one, at ≥ 3 % end-to-end.

**Outcome — the split answered "no bug", and the counters found a bigger
question.** `scan_extent_guard` is **0** on every cell: guard sub-plans reach
S1a.6.3's index perfectly, and the 10.8 % of self time in `facts_with` was
never an un-indexed scan. What the split showed instead is ownership —
**85.7 %** of `zebra -e`'s candidates and **100 %** of `features/05 -e`'s come
from guard sub-plans — and, with one more counter, that **71.8 %** of
`zebra -e`'s guard premises had every slot bound by the time the walk reached
them.

A premise with nothing left to bind is not a search. It asks whether one exact
proposition is in the KB; the fact store interns propositions, so at most one
fact can answer it and the store names that fact in a hash lookup, where the
scan fetched a ten-deep bucket and unified all of it. Identical by construction
— the pattern denotes one argument tuple, `unify` accepts a fact iff its
arguments *are* that tuple, no two facts share one — so the candidate sequence
is the same one-or-none sequence.

Candidates **1 172 870 → 238 567** on `zebra -e`, instructions 805 → 533 M,
end-to-end **−20.6 %** (and −22.3 % on `solve zebra`). The two puzzles differ
by 5× because `zebra`'s guard buckets are 9.96 facts deep and `zebra2`'s are
2.72. `branching/07 -e` prices the check itself at **+0.9 %** — it has no
ground premise anywhere and pays the slot inspection for nothing.
`scan_ground` / `scan_ground_guard` / `cand_ground` land with it, and they are
the first counters where the two engines stop doing the same work on purpose.

### Task T1a.6.12.4 — The semi-naive guard re-evaluation, at its measured reach

`Matcher::holds_seeded` **already exists** and is wired into
[`lookahead.rs`](../../../ein.rs/crates/ein-infer/src/lookahead.rs); the
boundary does not use it. Wiring it there is Win B's headline, and its ceiling
is now measured: 15.3 % of `zebra -e`'s evaluations are monotone, those
evaluations are 22.2 % of the run, so a *perfect* implementation saves at most
**3.4 %** — and only of the part T1a.6.12.1 leaves behind.

**Runs last, and may not run at all.** Build it only if T1a.6.12.1–3 leave the
queries dominant *and* it costs under a day; otherwise decline it in writing.

**Outcome — declined, and here is the writing.** Both sides of the product are
now measured, and no cell in the corpus has both:

| cell | monotone share of guard evaluations | `Matcher::holds` share of the run | ceiling |
|---|---:|---:|---:|
| `solve zebra -e` | 16.3 % | 13.7 % | **2.2 %** |
| `features/05 -e` | 11.1 % | 19.2 % | **2.1 %** |
| `features/01 -e` | **100 %** | **1.4 %** | **1.4 %** |

`features/01 -e` is the program design/06 was describing — every one of its
599 375 guard evaluations is monotone, the ≥ 80 % the design assumed — and its
boundary is 2.9 % of the run. The mechanism is not wrong about programs; it is
wrong about which programs have a boundary worth optimising.
Either way [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
closes here with the exhaustive table above, and
[design/06 § Win B](../design/06_saturation.md#4-win-b--a-semi-naive-boundary)
gets the outcome of all four of its refinements written next to their
projections.

### Task T1a.6.12.5 — The per-entering snapshot

**The measurement.** `Saturator::resume` is 10.3 % of `zebra -e` and 12.4 % of
`features/05 -e`; `Vec::clone⟨Entry⟩` alone is 3.5 % / 3.2 % of self time.
[S1a.6.9](s1a.6.9_fork_entry_delta.md) considered the `Arc`-layered snapshot
and **declined it at 0.6 %**, with the matcher at 80.5 %. It is 17× that share
now, and nothing about it changed: S1a.6.3 took 4.5× off the matcher, and a
constant cost became the third-largest block. This is Rule 6 doing its job.

`resume` deep-clones nine fields per entering — `engine`, `entries`, `queue`,
`parked`, `seen`, `guard_sets`, `matched_plans`, `pos_index`, `sym_rels` — of
which `entries` is a `Vec<Entry>` and each `Entry` owns three boxed slices, a
`BindingKey` and a stamp `Vec`.

**The blocker, and it is real:** a fork *mutates* entries in place. The
boundary writes `stamp` and `judged` on the very candidates it inherited, so
`Arc<Vec<Entry>>` with `make_mut` would copy on the first failed judgement,
which is most forks.

**The change:** split `Entry` into the immutable candidate (plan, disjunct,
regs, trail, premises, key — written once at enqueue and never again) and the
per-saturation judgement state (`stamp`, `judged`), which moves to a side table
the fork owns. Then the arena is genuinely immutable after enqueue and can be
shared behind an `Arc` with a push-only local overlay — the same layering
[design/03 §5](../design/03_data_model.md) uses for the KB, for the same
reason. `queue`, `parked` and `seen` get the same question asked of them
separately; `pos_index` and `matched_plans` are rebuilt from the plan list and
may not need carrying at all.

**Gate:** T0 and T1 identical (this is a fork's *state*, so the risk is a
resumed saturation that reaches a different fixpoint —
`a_resumed_saturation_reaches_the_same_fixpoint` is the unit test and the
corpus is the real one); ≥ 5 % on both `zebra -e` and `features/05 -e`. The
blind cells matter most here: 384 167 enterings each pay one `resume`.

While here: `Snapshot::new_facts_of` walks the fork's whole fact set with a
hash probe per fact, once per entering, to compute a delta that is usually a
handful of facts. It is small on today's cells (617 root facts on `zebra`) and
it is O(|KB|) per entering by construction, so measure it in the same pass and
fix it if the blind cells say so.

**Outcome — landed, and the blocker had already been removed.** T1a.6.12.1a
moved `judged_at` off the row, so `Entry` was written once at enqueue and never
again; the arena is now two layers — the parent's behind an `Arc`, the fork's
appended locally, ids dense and stable across the seam — and the copy that
remains is one `flattened()` per *snapshot*, one per root saturation, against
111 enterings on `zebra -e` and 384 167 on `features/05 -e`.

`Saturator::resume` **10.3 % → 5.5 %** cumulative when it was measured (7.6 %
now, against a denominator T1a.6.12.3 shrank by another 20 %); absolutely,
7.9 → 3.6 ms on `zebra -e` and 455 → 232 ms on `features/05 -e`. End-to-end
−11.9 % / −8.7 %, and −4.3 % on `branching/07 -e`, which never enters the
boundary at all and forks 11 501 times — the number that says this is what a
fork costs rather than what a boundary costs.

What is left in `resume` is `seen`, `fired` and `pos_index`: append-only, so
the same layering applies, at 5.5 % total rather than the 10.3 % that made this
task worth a day. `Snapshot::new_facts_of` never surfaced above 1 % on any
profile and is not touched.

## Acceptance for this stage

- The two cones below their targets above, or a written account of which one
  is not and why.
- **Every T1 counter identical**, T3 472/473 and T2 239/240 with
  [D2](../divergences.md) the only cell, `./run_tests.sh` green.
- `guard_eval` / `guard_eval_monotone` land as counters (done — the
  measurement that chose this stage), and the `scan_*`-by-caller split lands
  with T1a.6.12.3.
- [Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate)
  **closed**, and [design/06 § Win B](../design/06_saturation.md#4-win-b--a-semi-naive-boundary)
  carries the measured outcome of each of its four refinements.
- [baseline.md](baseline.md) gains the stage's section; the S1a.6.1 instruments
  are re-run at the end (Rule 6), because whatever is left after this is what
  chooses [S1a.6.7](s1a.6.7_relever_matrix.md)'s re-levering.

## Notes

- **The order is deliberate**: T1a.6.12.2 first (it is free and it only gets
  more true), then T1a.6.12.1 (the largest, and it changes what .3 and .4
  measure), then .5 (independent of all of them), then .3 and .4 against
  whatever is left. Re-profile between .1 and .3 — the phase has been wrong
  about what comes next twice now, both times because it did not.
- **The blind cells are half the acceptance.** S1a.6.4 found that the four
  milestone targets never run the blind enumerator; both halves of this stage
  are measured on `features/05 -e` and `branching/07 -e` as well, and
  `branching/07` is the control with **zero** guards — a cell where the whole
  boundary is inert and any change here must be exactly free.
- The fuzzer ([S1a.6.6](s1a.6.6_differential_fuzzer.md)) guards this stage as
  it guards the others; the boundary is where a wrong answer would be *silent*
  rather than loud, because a skipped judgement does not crash, it just fails
  to admit a candidate that should have fired.
