# S1a.7.0 — The speculation audit

**Phase:** P1a.7 (Parallelism)
**Estimate:** 1 day
**Depends on:** [P1a.6](../p1a.6_performance/README.md)
**Checks:** [design/08](../design/08_parallelism.md) §2
**Measures:** [Q-M1a.7](../open_questions.md#q-m1a7--may---jobs--1-move-counters)

**Status: shipped 2026-08-20, and it moved the phase.** Added at phase start,
numbered `.0` for the same reason T1a.6.3.0 and T1a.6.4.0 were: it runs
*before* the stage it was written for. [S1a.7.2](s1a.7.2_parallel_enterings.md)
is four days of refactor whose shape depends on one number nobody had measured
— how often a speculated entering would have been wrong — and that number is
measurable **without a single thread**.

| finding | number |
|---|---|
| the control — case-1 speculations, both arms forking the same root | **1 078 154, zero differences** |
| the re-validation rate, corpus-wide | **0.1 %** (550 of 1 078 704) |
| the re-validation rate, on the puzzles the milestone's targets name | **36–50 %**, and 97.2 % on `zebra2-hints -e` |
| speculations that returned the **wrong verdict** for their entering | **35** — `alive` where the sequential engine says `dead-post` |
| where every one of them is | **layer 1**, on the three zebra-family files |
| design/08 §2's "case 1 … is the whole of layer 1" | **inverted** — layer 1 is where every case 3 lives, and layers ≥ 2 have none |
| share of enterings in layers ≥ 2, on the workloads with a search worth parallelising | **98.2–99.9 %** |
| share of enterings in layers ≥ 2, on `zebra2 -e` — the phase's stated scaling target | **44.6 %** |
| the answer under 4 candidate orders and 3 integration policies, 16 files + 2 deep searches | **identical** — verdict and model set |
| what a whole-layer barrier costs on the workloads that want cores | **nothing** — 5 173 → 5 173 and 11 501 → 11 501 enterings |
| what it costs on `zebra2 -e` | **6.1×** the enterings — the prune it defers |
| and one nobody was looking for: `branching/07 -e` under a barrier | **1 135 → 406 ms** at `--jobs 1`, root depth **164 → 3** |

Numbers, method and the raw tables:
[scaling.md §1–§4](scaling.md#1-amdahl--what-a-solve-spends-its-time-on).

## Context

[design/08](../design/08_parallelism.md) §2 fans a layer out from
`R0 = Arc::clone(root_core)` — root as it stood when the layer opened — and
then commits the results **in candidate order**, validating each against the
write set `W` that the commits before it produced. Three cases, of which only
the third costs anything:

1. `W = ∅` — accept as computed;
2. `c` meets `{h : (not h) ∈ W}` — emit `dead-pre` from the clash, no
   saturation;
3. otherwise — **continue** the fork's saturation with `W` as the delta.

The phase's acceptance asks for a case-3 rate of "≤ a few percent", and
[Q-M1a.7](../open_questions.md#q-m1a7--may---jobs--1-move-counters) parks the
whole deterministic-parallel promise on it. Nothing about that question needs
threads: run the sequential engine, and beside every entering run the *same*
entering against `R0`. Where the two agree the speculation would have stood;
where they disagree, something would have had to correct it.

So the instrument comes before the refactor. This is the same order
[S1a.6.1](../p1a.6_performance/s1a.6.1_profile_baseline.md) put the profile in
before P1a.6 changed anything, and the same reason: a four-day stage whose
premise is unmeasured is four days of hoping.

## What it found

### 1. The control passed a million times

Case 1 is run through the comparison even though both arms fork the same root
by construction. It cannot differ; a build in which it does has a
nondeterminism the audit would otherwise have blamed on `W`. **1 078 154
case-1 enterings over 69 corpus entries, zero differences** — which is also the
first corpus-scale measurement of the property level 1's whole safety argument
rests on: `try_commitment_set` is pure with respect to root
([P1.21 R2](../../m1_engine_hardening/README.md)). `commitment.rs`'s
`two_enterings_share_no_mutable_state` asserted it on one fixture; this asserts
it on every entering the corpus has.

### 2. The rate is 0.1 % corpus-wide and 36–50 % where it matters

| run | case 3 | rate |
|---|---:|---:|
| `solve -e examples/zebra2-hints.ein` | 35 / 36 | **97.2 %** |
| `solve -e examples/zebra2.ein` | 50 / 101 | **49.5 %** |
| `solve -e examples/zebra.ein` | 49 / 111 | **44.1 %** |
| `solve examples/zebra.ein` | 6 / 13 | 46.2 % |
| `solve -e examples/branching/07_lookahead_off.ein` | 202 / 11 501 | 1.8 % |
| every other run (65 of 73) | 0 | 0 % |

The corpus average is 0.1 % and it is **not the number the acceptance criterion
means**. Written as "the re-validation rate ≤ a few percent", the criterion
passes on this corpus while being wrong on every workload a reader of the
milestone would recognise. It is restated per-workload in the phase README.

### 3. The speculation is not stale — it is wrong

On **35** enterings the speculative arm returns `alive` where the sequential
engine returns `dead-post`. The mid-layer `(not h)` is not bookkeeping: it is a
**premise** of `std.elim`, whose two rules read exactly this shape —
`domain-elimination` has a `(forall ?v_other … (not (?R ?a ?v_other)))` premise
and *asserts a positive* when every other value is excluded, and
`no-room-left` asserts `(false)` when every value is. A fork that lacks the
accumulated `(not h)` facts derives neither. One example,
`solve -e examples/zebra.ein` layer 1 entering 11, `|W| = 2`:

```
commitment    (co-located Englishman House-5)
sequential    dead-post, 603 facts, 382 firings
speculative   alive,     590 facts, 370 firings
derived only by the sequential fork
              (co-located Dog House-4)  (co-located House-3 Japanese)  …34 more
```

So case 3's continuation is load-bearing rather than a formality, and a design
that skipped it — or a read-set filter that decided these forks were unaffected
— would change `enterings_alive` / `enterings_dead_post`, the no-goods emitted,
the writeback set and the next layer's candidates. That is a T1 failure, not a
narration one.

### 4. design/08 §2's case-1 claim is inverted

The doc says case 1 "is the whole of layer 1, where every learned clause is the
candidate itself, so a writeback can only concern the candidate that just
died". Two things are wrong with that, and they are different mistakes.

**Which layers have a `W`.** The search is a cardinality BFS — layer *L*
enters commitment **sets of size L** — so a dead commitment licenses a clause
of width *L*, and a clause is not a fact. Only at *L = 1* is it a **unit**
clause whose negation *is* a fact root can hold, which is why both engines
guard the writeback on the commitment's length (`solve.rs:908` `c.len() == 1`;
`_helpers.py:454` `len(c) == 1`) and why clause minimisation would not change
it — neither engine minimises below the commitment
(`learned_clause = frozenset(c)`). Therefore:

- **layer 1 is the only layer that adds a fact to root mid-layer** — the
  `writeback` events by layer are `{1: 32}` on `zebra2 -e`, `{1: 31}` on
  `zebra -e`, `{1: 162}` on `branching/07 -e`;
- **every layer ≥ 2 is pure case 1** — 45 of 45 on `zebra2 -e`, and the
  validator has nothing to do there at all.

The **no-good store** is written mid-layer at every level and shared by `Arc`,
but it is not a hazard: no fork reads it while saturating — only
`generate_layer` at layer start and `emit_nogood`'s subsumption check at
commit time do, and `emit_nogood` takes `&Kb`.

**What a writeback reaches.** "A writeback can only concern the candidate that
just died" is true of *which candidate the clause names* and false of *what a
later fork derives from it* — and that difference is the whole of case 3, and
of finding 3 above.

### 5. The answer survives both things a parallel layer does to it

A parallel layer does two things the sequential engine does not: it enters
candidates in an order nobody chose, and it integrates what they learned
*late*. Both are invariance claims about the **answer**, and neither needs a
thread to test.

- **Order.** `lex`, `score-sum` and two seeded shuffles over 16 files: same
  verdict, same model set. The claim is not new — it is what `--shuffle` has
  always asserted (Q-M1a.5) — but it had only ever been exercised through the
  traversal-parity sweep, which compares ein.rs to ein.py rather than a run to
  another run of itself.
- **Integration time.** `SolveOptions::integrate_every` — `None` (sequential),
  a barrier every 4 enterings, and one barrier per layer. Same verdict, same
  model set, on the same 16 files, and on two five-layer searches of 5 173 and
  11 501 enterings.
- **And composed**, which is what a parallel layer actually does: a shuffled
  order *and* a whole-layer barrier. Same answer.

The argument for why — with the commutation identity, the monotonicity that
makes a death under a smaller root a real death, and the one case that is
*provisional* — is
[design/08 §2a](../design/08_parallelism.md#2a-deferred-integration--the-batch-synchronous-layer).
The short form:

> **A death found under deferred integration needs no re-check. A *solution*
> does.**

`integrate_every` does not yet re-check, so the model set is **measured** equal
rather than equal by construction. That re-check is one re-entry per recorded
solution node at the barrier, and it is
[S1a.7.2](s1a.7.2_parallel_enterings.md)'s to build.

### 6. Deferring costs the prune it defers — and on a deep search it costs nothing

| workload | sequential | whole-layer barrier |
|---|---:|---:|
| `zebra2 -e` | 101 enterings | **617** |
| `zebra -e` | 111 | **617** |
| `branching/06 -e` (0 writebacks) | 5 173 | **5 173** |
| `branching/07 -e` (162 writebacks) | 11 501 | **11 501** |

The zebras pay 6.1× and 5.6×, and batching at 20 gets them back to 1.1× and
1.7×. The deep searches pay nothing at all — the same split finding 4 reaches
from the other side, since layers ≥ 2 have no root write for a barrier to
delay.

### 7. A root write costs every later fork a layer — 2.8× on `branching/07 -e`

Not what the experiment was looking for. `Kb::fork` seals root's top layer so
the parent's later appends land in a new one, and **every fork inherits the
whole stack**. `branching/07 -e`'s 162 mid-layer writebacks leave root at
**depth 164**, and all 11 501 forks walk it:

| policy | enterings | root depth | wall |
|---|---:|---:|---:|
| sequential | 11 501 | **164** | **1 135 ms** |
| barrier every 20 | 11 501 | 13 | 447 ms |
| one barrier per layer | 11 501 | **3** | **406 ms** |

Same enterings, same answer, **2.8×** — at `--jobs 1`. It is pinned by
`deferring_collapses_roots_layer_stack`, and it is a P1a.6-shaped result that a
P1a.7 correctness experiment walked into. Whether the *sequential* engine
should coalesce is not this stage's call: it changes the traversal, so it is a
`--jobs`- or `--unordered`-scoped decision.

### 8. The workloads worth parallelising are not the ones the phase targets

| workload | e2e | enterings | layers | layer ≥ 2, enterings | layer ≥ 2, firings |
|---|---:|---:|---:|---:|---:|
| `zebra2.ein -e` | 31.1 ms | 101 | 2 | 44.6 % | 42.3 % |
| `zebra.ein -e` | 46.9 ms | 111 | 2 | 49.5 % | 53.2 % |
| `branching/06 -e` | 211.7 ms | 5 173 | 5 | 99.2 % | 99.6 % |
| `branching/07 -e` | 906.6 ms | 11 501 | 5 | 98.2 % | 99.8 % |
| `sq-bwd/houses -e` | 271.3 ms | 21 699 | 5 | 99.9 % | 100.0 % |
| `features/01 -e` | 1 856.2 ms | 384 167 | 5 | — | — |

Read the two halves together. The workloads with **≥ 98 % of their enterings in
layers ≥ 2** are exactly the ones where the speculation is exact by
construction and the validator is dead code — and they are also the only ones
slow enough for a core count to matter. The two zebras, which the phase's
scaling target names, are 29–47 ms runs that put **42–53 % of their firings in
the one layer that has the dependency**.

This is [S1a.6.4](../p1a.6_performance/s1a.6.4_hypgen_and_lattice.md)'s lesson
arriving a phase later: the targets were written against one shape of workload.
The phase README re-aims them.

## Acceptance

- The audit changes nothing about the run it audits: `stdout` and the whole of
  `--json-summary` byte-identical to the shipping binary, with the audit armed.
  ✅ — checked on `zebra2 -e` against `target/release/ein`, and the control in
  §1 is the corpus-scale version of the same claim.
- The re-validation rate is reported per run and corpus-wide, with the case-2
  and case-3 counts separated. ✅
- The three validation cases are classified by *what root actually gained*,
  not by the one write site the design predicted. ✅ — `W` is a set difference
  against `R0`'s fact set, so a mid-layer write nobody anticipated is counted
  rather than missed.
- What the audit compares is split into `kind` / `core` / `state`, and `state`
  is split alive from dead, for
  [`fork_delta_verify`](../../../utils/fork_delta_verify.py)'s reason: a dead
  fork under `enable_fail_fast_fork` stops at the firing that killed it, so its
  state is a firing-order-dependent prefix and not a fixpoint claim. ✅
- The two properties a parallel layer needs — order-invariance and
  integration-time-invariance of the **answer** — are **tests**, not
  paragraphs, and they run with the ordinary suite. ✅ — 6 tests, 2.15 s.
- The shipping path is untouched: `integrate_every = None` is T3-clean against
  ein.py on the whole corpus. ✅ — 496 identical cells, 2 differing, both of
  them [D2](../divergences.md)'s two known shapes.

## Tasks

### Task T1a.7.0.1 — The instrument

`ein-infer/src/spec_audit.rs`, behind `--features spec-audit`, inert unless
`$EIN_SPEC_AUDIT` names a file — the shape
[`fork_audit`](../p1a.6_performance/s1a.6.9_fork_entry_delta.md) established.
`LayerAudit::start` takes `R0 = root.fork()` at layer start; `check` runs
`try_commitment_set` against it for every candidate and compares with the
result the sequential loop just got, writing one JSON-Lines record per
entering.

Two details that are the difference between a measurement and a story:

- the speculative arm gets `Events::off()`, so it cannot narrate into the run's
  event log;
- both arms live in one process and one `Terms`, so `FactId` is directly
  comparable and the comparison is a set difference — where `fork_audit`, which
  crosses processes, has to render every fact to an s-expression first.

### Task T1a.7.0.2 — The corpus sweep

[`utils/spec_audit.py`](../../../utils/spec_audit.py), modelled on
`fork_delta_verify.py`: the `solve` and `solve -e` runs of every `positive` and
`stdlib` corpus entry, aggregated into the tables above. `--no-fail-fast`
appends `(config :enable-fail-fast-fork false)` so a dead fork's *fixpoint* is
compared rather than its prefix. Exit code 1 when any `kind` moved, because
that is the number the phase's contract is written against.

### Task T1a.7.0.3 — The layer profile

The `enter` and `writeback` events, counted by layer, for the two zebras and
the three deep-search entries. This is finding 4's evidence and finding 5's
whole substance, and it costs one `--events` run per workload.

### Task T1a.7.0.4 — Order-invariance, as a test

`ein-infer/tests/search_invariants.rs`. Four traversals of each of 16 files —
the canonical `lex` order, the `score-sum` order (a *different* deterministic
permutation, not a random one), and two seeded shuffles — compared on verdict
and model set. Models go out as sorted s-expressions: each run reloads the file
(a solve *writes* to root, so a KB cannot be re-used), so each has its own
`Terms` and an id comparison would report a difference where there is only a
different interning order.

### Task T1a.7.0.5 — Deferred integration, as a mode and a test

`SolveOptions::integrate_every: Option<usize>` — the barrier policy. `None` is
the sequential engine and the only shipping value; `Some(n)` holds an
entering's root writes back and applies them every *n* enterings and at every
layer end.

It buffers exactly two things, because they are the only two an entering writes
to root: the learned clause and the singleton `(not h)` writeback. Everything
else an entering produces is fork-local or lives in `LoopState`. That is what
makes the mode ~40 lines rather than a rewrite of the layer loop — and what
keeps `None` byte-identical, since it takes the same branch it always did.

It is an **execution** knob and deliberately not a `SolverConfig` field: a
`(config …)` block in a puzzle file must not be able to set it, and
[S1a.7.5](s1a.7.5_jobs_contract.md) T1a.7.5.1 makes the same call for `--jobs`.

The test runs three policies × 16 files, plus two five-layer searches, plus the
composition with a shuffled order; `ein-infer/examples/defer_probe.rs` is the
measurement that produced finding 6 and 7's tables.

## What it changes

- **[S1a.7.2](s1a.7.2_parallel_enterings.md) splits in two.** Layers ≥ 2 are
  parallel with **no validator at all** — provably, because no root write
  happens mid-layer there — and layer 1 needs a decision that is now a decision
  between measured options rather than an assumption. The stage's acceptance
  gains the proof obligation that the "no root write in layers ≥ 2" property is
  *asserted* in the engine, not just observed here.
- **Q-M1a.7 is no longer open in the dark.** The recommendation it carries — "no
  counter movement, plus an opt-in escape" — survives; what changes is the cost
  of keeping it, which is now a per-workload number.
- **The phase's scaling target moves** off `zebra2 -e` and onto the entries
  that have a search. A 29 ms run whose parallel half is 11 ms cannot show 6×
  whatever the engine does.
- **[design/08](../design/08_parallelism.md) §2 is corrected** where it is
  wrong, with this stage cited, and gains a **§2a** for the shape a parallel
  layer actually has — deferred integration, its commutation identity, and the
  one verdict that stays provisional under it.
- **A fourth option is on S1a.7.2's table, and it is the measured one.**
  Batch-synchronous integration is free on the workloads that want cores,
  costs the deferred prune on the ones that do not, and is 2.8× *faster* on
  `branching/07 -e`. What it does not yet have is the barrier re-check that
  turns "the model set is measured equal" into "the model set is equal".

## Notes

- The audit doubles the enterings of the run it audits and writes a line per
  entering, so it is not for the corpus's largest entries; the sweep runs each
  cell under a timeout and counts what it reached. `features/01 -e` at 384 167
  enterings is measured for its layer profile, not its audit.
- `--no-fail-fast` is the arm that would separate "the fork reached a different
  fixpoint" from "the fork stopped at a different firing". It is *not* what the
  headline numbers above were taken under, and the split matters for the
  `core` column specifically — see [scaling.md §3](scaling.md#3-the-audit).
- **`integrate_every` has no CLI flag and is not meant to have one yet.** It is
  reachable from `SolveOptions` — which is what the tests and the probe use —
  and the flag that exposes it is `--jobs`'s, in
  [S1a.7.5](s1a.7.5_jobs_contract.md). Adding a flag now would put a mode on
  the `--help` surface (a parity surface, Q-M1a.13) before the phase has
  decided what the mode is called or what it promises.
- **The invariance tests are exhaustive-only, deliberately.** Under
  `stop_after` the claim is about a *prefix of a traversal*, and a deferred
  layer records a solution node from a provisional alive verdict — which is
  exactly the case design/08 §2a says needs a barrier re-check. Testing it
  before that re-check exists would either pass by luck or fail for the reason
  already written down.
