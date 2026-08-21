# S1a.6.9 — The fork-entry delta (the resumed saturator)

**Phase:** P1a.6 (Performance)
**Status:** **shipped 2026-08-19.** T1a.6.9.1–3 landed the instrument, the
mechanism and the evidence; [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
was then answered **ein.rs-only**, so T1a.6.9.4 flipped the resumed saturator
on and gave `--trace` its root-saturation section.
[D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)
records the divergence; [S1a.6.10](s1a.6.10_parity_contract.md) teaches the
harness to hold it and [S1a.6.11](s1a.6.11_fixture_goldens.md) replaces what
it stops comparing.
**Estimate:** 3 days (1 d measure + decide, 2 d conditional implementation)
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md) — and it is the
*upper bound* [S1a.6.3](s1a.6.3_beta_memories.md) is chasing, so the two
are read together.
**Was gated on:** [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
— this stage could not land its own headline change until that question was
answered, because the change is **observable**. Answered 2026-08-19.
**Relates to:** [design/06](../design/06_saturation.md) §4–§5,
[design/05](../design/05_matcher.md) §7,
[F11](../../followups/f11_deductive_layer_perf.md) D1

> **Instruments (M1a [S1a.10.6](../p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `count_work.py`. It is gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../../utils/README.md#the-census).

## Context

The engine already indexes plans by relation and drives the closure from
the delta: `Saturator::rebuild_index` builds `rel → [plan]` for every
top-level positive premise, `enqueue_pass(Some(delta))` seeds each delta
fact into exactly those plans, and `run_seeded` starts the match *at* the
new fact instead of rescanning the extent. That is D2 + D5, it is in both
implementations (`_pos_index` / `pos_index`), and design/06 §4 opens by
stating it: *"the closure is already semi-naive."*

**It is not applied at the one boundary where the delta is smallest.**
`commitment::try_commitment_set` forks the saturated root, writes the
commitment's `k ≤ 5` hypothesis facts, and then builds a **fresh**
`Saturator` — fresh engine, fresh plan cache, empty `seen`, empty `fired`,
empty `parked`, `delta = None`. `delta == None` is a FULL pass, so the
first thing every entering does is full-match every plan against a KB it
inherited at its own fixpoint. The parent's entire deductive closure is
then re-derived, firing by firing, so that the saturator can discover
that each conclusion is already present.

Every entering forks the *root* — the lattice is a cardinality BFS, not a
chain — so there is exactly one parent state, it is at quiescence, and
the delta is literally the commitment set.

## The measurement

`--events --events-level verbose`, `master` @ `fe62f94`, the S1a.6.1
machine. Root and per-fork totals separated at the `enter` events;
[baseline.md §9](baseline.md#9-the-fork-entry-re-derivation) carries the
full tables and the one-line command.

| `-e` run | enterings | fork firings | **redundant** | productive | fork enqueues |
|---|---:|---:|---:|---:|---:|
| `zebra2` | 101 | 38 136 | **36 442 (95.6 %)** | 1 694 | 81 766 |
| `zebra` | 111 | 113 746 | **107 610 (94.6 %)** | 6 136 | 198 763 |

*(T1a.6.9.1 corrected two attributions in the first printing without moving
the headline — `utils/fork_split.py`, and the note in
[baseline.md §9](baseline.md#9-the-fork-entry-re-derivation).)*

And the enclosing share, `utils/profile_ein_rs.py --cum-of`:

| run | cumulative in `ein_infer::commitment` |
|---|---:|
| `zebra -e` | **95.0 %** |
| `zebra2 -e` | **86.7 %** |

So on `zebra -e` — *the one workload that misses its target*, at 1.46× —
95 % of the run is inside fork saturation and 95 % of what fork saturation
narrates is the root's own fixpoint, re-derived 111 times. This is a
larger single lever than anything in [§7](baseline.md#7-the-top-five-costs):
item 1 (plan re-compilation, 21.1 %) is a *consequence* of the same fresh
saturator, and item 3 (the matcher, 66.9 % of `zebra -e`) is where the
re-derivation is actually paid.

## Why it is not free — the observables it moves

A redundant firing is a `Firing`, and a `Firing` is narrated:

- **T2 (`--events` at `verbose`)** emits a `fire` line per firing with its
  `redundant` flag. `docs/kernel/inference/events.md` § Levels says T2 runs at
  verbose *specifically* because "a dropped redundant firing is exactly the
  kind of difference a port introduces and the tier exists to catch."
- **T3** compares `solve --trace {out}/trace.md` and
  `solve --dump-states {out}/states`, which are corpus runs on the zebra
  entries. `trace/linearize.rs` reports `n_firings = p.firings.len()`;
  `dump/lattice.rs` and `dump/state.rs` write `("firings", len)` per node;
  `render/shape.rs` renders `firings.iter().take(5)` — and under this
  change those would be a *different* five.
- **T0/T1** are safe: `BaseStats` counts enterings, saturations, merges
  and no-goods, and never a firing.

So this is not a P1a.6 change under [Rule 1](README.md#rules-for-this-phase).
It is a change to what the engine *says it did* — and therefore a decision
about the M1 engine that both implementations take together, or not at all.

## What is *not* at risk

> **Verified 2026-08-18 (T1a.6.9.2), and claim 2 is wrong.** 1 and 3 hold
> exactly, over 1.08 M enterings of the whole corpus compared fact by fact;
> the provenance graph does **not** survive — 90 002 facts get a different
> *primary* justification, because a resumed fork's inherited parked
> candidates carry root's tiebreakers and the boundary admits one per round.
> The mechanism is admission *order*, which no amount of argument about
> duplicate rejection reaches, and which cannot be designed away: matching a
> fresh pass's numbering needs a fresh pass.
> [baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured) has
> the sweep; the argument as originally written is kept below because the
> half that survived is the half the fixpoint claim rests on.

The argument that the verdict, the models and the provenance graph all
survive, which T1a.6.9.2 has to verify rather than accept:

1. **The fixpoint.** The root is at quiescence, so every match over
   root-only facts was already enqueued and fired there. A match that is
   new in the fork uses at least one fork-local fact — a commitment fact
   or something derived from one — and is therefore reachable by seeding
   the delta. Inheriting `fired` and `seen` skips exactly the matches that
   already fired, and D5 finds exactly the rest.
2. **Alternative justifications.** `alt` is emitted when
   `Kb::record_justification` returns `true`, and the fork reads
   `alternatives(fact)` through the layered view, so a duplicate of a
   root-recorded justification is already rejected. Measured on
   `zebra2 -e`: 5 111 `alt` records, 5 015 of them inside forks, **all**
   from a redundant firing (the first printing's "4 335 after a redundant
   firing, 776 after a productive one" was an attribution artefact — `alt`
   is emitted *before* its own `fire` line). 4 317 of the 5 015 are recorded
   by a firing whose **premises** include a fork fact while its
   **conclusion** is inherited, and those are delta-reachable by
   construction. **698 are not**: premises and conclusion both pre-date the
   fork. Those are reachable only through the inherited *parked* set, and
   they are the reason this claim is verified rather than argued. The other
   ~31 000 redundant firings record nothing.
3. **The boundary.** A parked candidate is one whose guard *failed*, and
   the KB is append-only, so an `(absent P)` that failed cannot start
   passing. The root's parked set is inherited as a set of candidates that
   still fail — which is what the watch stamp already encodes.

## Tasks

### Task T1a.6.9.1 — Land the measurement ✅

**Shipped 2026-08-18** as `utils/fork_split.py`, which corrected two
attributions in §9's first printing without moving the headline.

Fold the tables above into [baseline.md §9](baseline.md#9-the-fork-entry-re-derivation)
and make the split re-runnable from one command, like every other
instrument in the phase (`utils/count_work.py` is the natural home — it
already parses nothing and counts what the engine did; this adds a
`--events`-fed fork split, or a small `utils/fork_split.py` if that fits
badly). Re-run it at the end of every stage in the phase, because
[S1a.6.8](s1a.6.8_compile_cache_and_extents.md) removes the compile share
of exactly this cost and the ratio will move.

### Task T1a.6.9.2 — Verify the three invariants, offline ✅

**Shipped 2026-08-18.** `Saturator::resume` + `ein_infer::fork_audit` behind
`--features fork-delta`; `utils/fork_delta_verify.py` runs one binary twice
over every `solve`-family run of every `positive` / `stdlib` corpus entry.
**Result: the fixpoint and the boundary hold; the provenance graph does not.**
That is the counter-example this task asked for — and it did not end the
stage, because everything the *verdict* rests on survived.

Before proposing anything: build the resumed saturator behind a
`fork-delta` feature flag that is **off by default**, and check the three
claims above by comparing artefacts that are *not* firing lists —

- the fork's fact set at quiescence, fact by fact (`state_key` equality
  per entering, every entering, whole corpus);
- the full alternatives map per fact, dumped and diffed;
- the verdict, `k`, the models, the unsat core, the no-good clauses.

If any of those move, the idea is wrong and this stage ends here with the
counter-example written down. That is a successful outcome.

### Task T1a.6.9.3 — Answer Q-M1a.18 with a diff, not an argument ✅

**Shipped 2026-08-18.** The rendered before/after is
[fork_delta_trace.md](fork_delta_trace.md); the sizes (T2 at both levels, the
T3 cells, the trace) are
[baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured).
Q-M1a.18 is restated there against what was measured rather than what was
argued, and it is **not** answered here.

With the flag on, produce the *size* of the divergence the decision is
about: the T2 line-count delta, the T3 cells that move, and a
before/after of `solve --trace examples/zebra2.ein` so the question is
decided against a rendered human trace rather than a count. The case for
changing both engines is that the trace gets **better** — a hypothesis's
proof should show what the hypothesis *added*, not 960 re-derivations of
what was already true, which is the standard
[`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
sets and what
[`08-human-style-deductive-trace`](../../ideas/08-human-style-deductive-trace.md)
asks for.

### Task T1a.6.9.4 — The resumed saturator ✅

**Shipped 2026-08-19**, and two of the three things it was expected to do
turned out to be the wrong work:

- **the flip** — `Saturator::resume` is the shipping path. The escape hatch is
  a `fork-delta` build with `EIN_FORK_DELTA=0`, kept because
  [D3](../divergences.md) needs both arms out of one binary to stay measured.
- **the `Arc`-layered snapshot: not built, and that is a measurement.** The
  prototype removed 77 % of the firings for 34 % of the time and the deep copy
  was the obvious suspect. It is not: `perf` puts `Vec::clone<Entry>` at
  **0.6 %** and the whole `fork/copy` subsystem at 0.1 %. What is left is the
  matcher, at **80.5 %** of `zebra -e` — which is
  [S1a.6.3](s1a.6.3_beta_memories.md)'s subject, not this stage's. Building
  the layered snapshot would have been the wash [Rule 3](README.md#rules-for-this-phase)
  exists to prevent.
- **the ein.py half: not done, by decision.** Q-M1a.18 was answered ein.rs-only.
- **the renderer change: done, and it was the important one.** `--trace` gains
  a *Before any assumption* section — root's own 321 steps, then
  `Assuming …`, then the 240 the hypothesis adds, numbered as one sequence.
  Without it the solution's proof silently lost every rule that fires only at
  root, which is what `test_idea08_acceptance` catches and what
  [T1a.6.11.2](s1a.6.11_fixture_goldens.md) ports to ein.rs.

If the answer is yes, the mechanism is small because every piece exists:

- snapshot the root saturator's `engine`, `seen`, `fired`, `parked` and
  tiebreaker high-water mark after root saturation (and re-snapshot after
  every mid-layer singleton writeback / forced positive re-saturation —
  `solve.rs` phase 2 already re-saturates root there);
- `Saturator::resume(snapshot, delta)` instead of `Saturator::new`, with
  `delta = hypothesis_facts`;
- share the snapshot by `Arc`; the fork's own additions are its delta, as
  with the KB.

ein.py gets the same change first, since it is the oracle. Both land in
the same commit pair, with the T2/T3 goldens regenerated once and the
reason recorded in [divergences.md](../divergences.md).

### Task T1a.6.9.5 — If the answer is no: the salvage — **moot** ✕

The answer was yes. The number stands as
[S1a.6.3](s1a.6.3_beta_memories.md)'s target anyway, re-aimed: the fork
boundary is no longer where the re-derivation is, so what beta-memories have
to make free is the **77 %** of a *resumed* fork's firings that are still
redundant — inside its own delta, where a symmetric-transitive rule set
ping-pongs.

**Number for S1a.6.3's acceptance, measured either way:** a resumed fork does
**9 834** firings on `zebra2 -e` where a fresh one does 38 136, and **26 656**
against 113 746 on `zebra -e`. Root beta-memories have to make that difference
nearly free *without* changing the firing sequence.

Two parts of the win are available without touching the narration, and
they are worth taking either way:

- **the compile share** — already
  [S1a.6.8](s1a.6.8_compile_cache_and_extents.md), which is why it runs
  first;
- **the match share** — [S1a.6.3](s1a.6.3_beta_memories.md)'s *root*
  beta-memories are precisely "compute the root's matches once and replay
  them into every fork", which produces the same firings in the same order
  and is therefore invisible. This measurement is the number that stage
  should be judged against: it does not have to make matching faster, it
  has to make **the 95 % that is re-derivation** nearly free.

### Task T1a.6.9.6 — Re-measure and record ✅

**All four targets are met**, and the one that needed the stage is
`solve zebra.ein -e`: **397.2 ms** against ≤ 400 ms, from 539.9 at S1a.6.8.
`solve zebra2.ein -e` is **99.1 ms** against ≤ 200. Both are `utils/e2e_baseline.py`
*process* measurements, which is what the milestone's targets mean.

| instrument | where |
|---|---|
| the four targets | [README § Targets](README.md#targets) |
| what the fork does now | [baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured); §9 is marked historical |
| the profile | `zebra -e` is **80.5 % matcher**, up from 72.6 — the fork boundary's share went *to* the join |
| the ledger | [design/README § Measured](../design/README.md#measured) |
| the gates | `cargo test` green, `./run_tests.sh` 1 506 + 21 green, T3 465/473 with D2 and D3's seven cells |

**Six test comparisons had to be narrowed to keep the suites green**, and
that is the honest cost of landing the engine change before the stage that
relaxes the contract. Each cut is narrow and documented at its site, but they
were made one at a time, each revealed by the next test to go red:
`hypgen_parity` (firing counts → the event ordinal → a `dead-post` core),
`dot_parity` (the `slice` view), `dump_parity` (the timeline's `firings` → the
`enterings/` subtree → the snapshot's dead state keys → the lattice DOT
rendered from them), `trace_parity` (the rendered trace itself). They are tabulated in
[D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it),
where the chain is also read as one sentence — *a fork's derivation, and
anything keyed on a dying fork's stopping point, is narration* — which is what
[T1a.6.10.0](s1a.6.10_parity_contract.md) writes down once and implements
once, deleting all six.

The task as originally written:

The phase rule: re-run the S1a.6.1 instruments, record in
[design/README.md § Measured](../design/README.md#measured), and state
which of the four targets moved.

## Acceptance

- ✅ The fork split is in baseline.md and re-runnable by one command —
  `utils/fork_split.py`, which corrected two attributions on the way in.
- ✅ The three invariants of § What is *not* at risk are **verified**, not
  argued, on the whole corpus — 3 228 853 enterings, `utils/fork_delta_verify.py`.
  Two hold; the third does not, and the counter-example is recorded in
  [baseline.md §11](baseline.md#11-the-resumed-fork-saturator-measured) and
  [D3](../divergences.md).
- ✅ Q-M1a.18 is answered with a rendered before/after trace attached —
  [fork_delta_trace.md](fork_delta_trace.md).
- ✅ The answer was **yes, in ein.rs only** — so, adapted from the clause the
  plan wrote for "yes in both": ein.rs changed, ein.py did not, the divergence
  is ledgered rather than the goldens regenerated (there are no goldens — the
  harness diffs two live engines), and `zebra -e` is **397.2 ms** against its
  ≤ 400 ms target.
- ✅ The number is carried into [S1a.6.3](s1a.6.3_beta_memories.md)'s
  acceptance anyway, re-aimed at what is left rather than at what was removed.
- ➕ Not in the original acceptance, and needed: `--trace` renders root's own
  saturation, or the proof silently loses every rule that fires only there.

## Notes

- The same fresh-saturator shape is what makes item 1 of
  [§7](baseline.md#7-the-top-five-costs) cost 21.1 %: 12 625 of the run's
  17 250 `compile` events are inside forks, 125 per entering, all of them
  re-compiling plans the root already compiled (another 4 375 are hypgen's,
  which the corrected split separates). S1a.6.8 fixes the compile
  half by sharing the memo; this stage is the same observation applied to
  the *matching* half, and the reason they are separate stages is that
  only one of them is invisible.
- `alt` on `zebra -e` is **0** — the redundant firings there record
  nothing at all, which is why that puzzle shows the cost at its purest.
- This does not re-litigate the search layer ([Rule 4](README.md#rules-for-this-phase)):
  the branch count, the entering count and the traversal order are
  untouched. What changes is the cost of one entering — **and, once measured,
  the proof it records.** `summary.json` says the search is identical, field
  for field; the proof graph says which of a fact's derivations got there
  first, and that is not identical. Both statements are true and the second
  one is what Q-M1a.18 was really deciding.
- **What was expected to be the work, and was not.** The prototype removed
  77 % of the firings for 34 % of the time, so T1a.6.9.4 was scoped around
  the per-entering snapshot copy. `perf` put that at 0.6 %. Measuring before
  building saved the stage two days and handed the remainder to
  [S1a.6.3](s1a.6.3_beta_memories.md), where the matcher now sits at 80.5 %.
- **What was not expected to be the work, and was.** Landing the flip before
  [S1a.6.10](s1a.6.10_parity_contract.md) meant narrowing six cross-engine
  comparisons one at a time, each revealed by the next test to go red. Worth
  recording as an ordering lesson: a change that moves an observable wants the
  contract stage *first*, or it pays for the same relaxation twice.
