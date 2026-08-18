# S1a.6.9 — The fork-entry delta (the resumed saturator)

**Phase:** P1a.6 (Performance)
**Estimate:** 3 days (1 d measure + decide, 2 d conditional implementation)
**Depends on:** [S1a.6.1](s1a.6.1_profile_baseline.md) — and it is the
*upper bound* [S1a.6.3](s1a.6.3_beta_memories.md) is chasing, so the two
are read together.
**Gated on:** [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
— this stage may not land its own headline change until that question is
answered, because the change is **observable**.
**Relates to:** [design/06](../design/06_saturation.md) §4–§5,
[design/05](../design/05_matcher.md) §7,
[F11](../../followups/f11_deductive_layer_perf.md) D1

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
| `zebra2` | 101 | 37 647 | **35 996 (95.6 %)** | 1 651 | 80 892 |
| `zebra` | 111 | 112 762 | **106 657 (94.6 %)** | 6 105 | 197 125 |

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
  `redundant` flag. `conformance/EVENTS.md` § Levels says T2 runs at
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
   `zebra2 -e`: 5 111 `alt` records, 4 894 of them inside forks, 4 335
   following a *redundant* firing — i.e. the redundant firings that matter
   are the ones whose **premises** include a fork fact while their
   **conclusion** is inherited, and those are delta-reachable by
   construction. The other ~32 000 record nothing.
3. **The boundary.** A parked candidate is one whose guard *failed*, and
   the KB is append-only, so an `(absent P)` that failed cannot start
   passing. The root's parked set is inherited as a set of candidates that
   still fail — which is what the watch stamp already encodes.

## Tasks

### Task T1a.6.9.1 — Land the measurement

Fold the tables above into [baseline.md §9](baseline.md#9-the-fork-entry-re-derivation)
and make the split re-runnable from one command, like every other
instrument in the phase (`utils/count_work.py` is the natural home — it
already parses nothing and counts what the engine did; this adds a
`--events`-fed fork split, or a small `utils/fork_split.py` if that fits
badly). Re-run it at the end of every stage in the phase, because
[S1a.6.8](s1a.6.8_compile_cache_and_extents.md) removes the compile share
of exactly this cost and the ratio will move.

### Task T1a.6.9.2 — Verify the three invariants, offline

Before proposing anything: build the resumed saturator behind a
`fork-delta` feature flag that is **off by default**, and check the three
claims above by comparing artefacts that are *not* firing lists —

- the fork's fact set at quiescence, fact by fact (`state_key` equality
  per entering, every entering, whole corpus);
- the full alternatives map per fact, dumped and diffed;
- the verdict, `k`, the models, the unsat core, the no-good clauses.

If any of those move, the idea is wrong and this stage ends here with the
counter-example written down. That is a successful outcome.

### Task T1a.6.9.3 — Answer Q-M1a.18 with a diff, not an argument

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

### Task T1a.6.9.4 — The resumed saturator (conditional on Q-M1a.18)

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

### Task T1a.6.9.5 — If the answer is no: the salvage

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

### Task T1a.6.9.6 — Re-measure and record

The phase rule: re-run the S1a.6.1 instruments, record in
[design/README.md § Measured](../design/README.md#measured), and state
which of the four targets moved.

## Acceptance

- The fork split is in baseline.md and re-runnable by one command.
- The three invariants of § What is *not* at risk are **verified**, not
  argued, on the whole corpus — or the counter-example is recorded.
- Q-M1a.18 is answered with a rendered before/after trace attached.
- If the answer is yes: both engines changed, T3 green on regenerated
  goldens, `zebra -e` re-measured against its ≤ 400 ms target.
- If the answer is no: the number is carried into
  [S1a.6.3](s1a.6.3_beta_memories.md)'s acceptance as its target, and
  this stage closes as a measurement.

## Notes

- The same fresh-saturator shape is what makes item 1 of
  [§7](baseline.md#7-the-top-five-costs) cost 21.1 %: 16 875 of the run's
  17 250 `compile` events are inside forks, ~167 per entering, all of them
  re-compiling plans the root already compiled. S1a.6.8 fixes the compile
  half by sharing the memo; this stage is the same observation applied to
  the *matching* half, and the reason they are separate stages is that
  only one of them is invisible.
- `alt` on `zebra -e` is **0** — the redundant firings there record
  nothing at all, which is why that puzzle shows the cost at its purest.
- This does not re-litigate the search layer ([Rule 4](README.md#rules-for-this-phase)):
  the branch count, the entering count and the traversal order are
  untouched. What changes is the cost of one entering.
