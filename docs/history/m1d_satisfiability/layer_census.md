# The layer census — what a layer kills, and what the killing is worth

**Stage:** [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) — T1d.10.1.1 … T1d.10.1.4
**Taken:** 2026-08-24, commit `87f88f6` + this stage, `ein 0.1.0`
(`ein-events/1`), i9-14900HX P-core, `powersave`/turbo on
**Instrument:** [`utils/layer_census.py`](../../../utils/layer_census.py) — 180
corpus entries, one `solve -e` cell each, **360 child processes** (bare, then
narrated), 6 min wall
**Re-take:** `utils/bench_env.sh python3 utils/layer_census.py --layers --json census.json`
**Addendum 2026-08-26** — [§10](#10-the-re-take--2026-08-26-and-what-p1d2-and-p1d3-moved),
the re-take on the engine P1d.2 and P1d.3 left: every moved row is the two
fixtures P1d.2 added, the phase entry's layers reproduce to the digit, and two
crosses the first census could not take — **5 of 51** searching cells state an
obligation and **0 of the 25** that walk an exact powerset do, and **18.1 %** of
the swept enterings happen under a run `corpus.toml` declares.
**Addendum 2026-08-25** — [§4.1](#41-the-depth-10-probe--2026-08-25-and-depths-610-add-nothing),
a run of the obligations twin with the cap at **10**: 10 587 736 enterings, 15
minutes, **zero new models**. Not from the instrument, and it carries no
per-layer rows; what it settles is the size of `d_stop − d_found`, which the
default cap was hiding.

| finding | number |
|---|---|
| corpus entries swept | **180**, of which **49 reach the search** at all |
| enterings across them | **2 201 027** |
| candidates the prefix join proposed | **2 232 330** |
| …dropped because an element left `alive` | **0** — and it is [structural](#6-one-of-the-two-filter-arms-cannot-fire) |
| …dropped by a learned clause | **31 303 — 1.4 %** |
| cells where the clause store dropped **nothing** | **41 of 49**, holding **2 128 908** of the 2 201 027 enterings |
| cells that never learned a clause at all | **35 of 49** |
| cells whose enterings are **exactly** `Σₖ C(alive, k)` | **25 of 49** — **2 128 512 enterings, 96.7 % of the corpus's search work** |
| the phase's entry, past its last new model, at `-m 5` / `-m 10` | **92.1 % / 99.54 %** — [§4.1](#41-the-depth-10-probe--2026-08-25-and-depths-610-add-nothing) |
| cells where layer 1 kills something | **4** — `zebra`, `zebra2`, `zebra2-hints`, `branching/07` |
| cells where `alive` ever shrinks | **3**, all of them in that four |
| the clause store's yield where it works | 7.7 % … 47.3 % of a layer's join, and **rising with depth** |
| `zebra2-minus-15` at depth 4, measured by a budget probe | join **245 612**, clause-dropped **88 887 — 36.2 %** |

**The one-line reading.** The note this milestone is built on says the engine
*"enumerates a powerset because it has no way to say that something is
required"*. That is not a metaphor and it is not a tendency: **for 25 of the 49
corpus entries that search at all, the number of commitments entered is exactly
the sum of binomial coefficients** — `features/01_not_and_absent -e` enters
`C(35,1) + … + C(35,5) = 384 167`, term for term, and
`saturation/square-unique/terminus -e -m 3` enters `C(153,1..3) = 597 057`.
Those 25 cells are **96.7 % of all the search work in the corpus.** Nothing
died, so nothing was learned, so nothing was filtered, and the lattice was
walked.

**And the phase's premise needs one correction.** P1d.10 opens on *"nothing
prunes at layer 1, so layer 2 is the full `C(96,2)`"*, and asks for the layer-2
filter rate as a number. It is **0 %** — but not because the clause store is
inert. It is 0 % because at layer 2 the clause store **cannot** contribute:
every layer-1 clause is a singleton, and the singleton `(not h)` writeback plus
the inter-layer retain have already removed its element. The store's first
possible contribution is layer 3, and on `zebra2-minus-15` it is **26.8 %**
there and **36.2 %** at layer 4. The clause store works. It starts two layers
late, and by then the layer is 44 089 enterings.

---

## 1. The instrument

`nogoods_emitted` has always said what a layer's deaths *produced*. Nothing
said what the resulting clauses *removed*, which is the other half of the
sentence the phase rests on — and the half that decides whether a barren layer
is a curiosity or the whole cost.

T1d.10.1.1 adds it in two pieces.

**In the engine**, [`apriori::filter_reason`](../../../ein.rs/crates/ein-infer/src/apriori.rs)
returns *which* of the filter's two questions rejected a candidate rather than
a bare `bool`, and [`LayerCensus`](../../../ein.rs/crates/ein-infer/src/solve.rs)
accumulates a sixteen-column row per layer, emitted as a `layer`
event ([events.md](../../../docs/kernel/inference/events.md#layer--the-clause-yield-census)).
Three design points are worth stating because each was a choice:

- **The two `dropped_*` are attributed in check order.** A candidate can fail
  both questions; that counts as `dropped_dead`, so `dropped_nogood` means
  *every element still alive and a learned clause covered the set anyway*.
  Summing them would hide the only column anyone wants.
- **Everything else is a difference of two whole-run counters** taken at the
  layer's open and its close, so a counter added to `BaseStats` reaches the
  census without anyone re-deriving it — and `Σ entered = enterings_total` is
  an invariant a test checks rather than a hope.
- **No timing in the event.** A `ms` field would make the stream
  non-deterministic and the goldens unreadable, and
  [events.md](../../../docs/kernel/inference/events.md)'s fourth ground rule
  says an instrumented run is not a benchmark anyway.

**It is unconditional, not behind a feature**, which is a departure from
[`ein_core::counters`](../../../ein.rs/crates/ein-core/src/counters.rs)'s
discipline and the stage's own wording. The reason is measured: the counting is
two increments per candidate in a predicate that already allocates a `Vec` and
walks the entire clause store per candidate, and on `zebra2-minus-15 -m 3` —
48 745 enterings, 60 260 filtered candidates against 11 577 clauses — it is
**under the run-to-run noise** (§9). A counter that has to be asked for is a
counter no corpus sweep has, and this one's whole purpose is the sweep.

**Outside the engine**, [`utils/layer_census.py`](../../../utils/layer_census.py)
runs every corpus entry under `solve -e` **twice**: bare, for the wall clock
and the peak RSS, and narrated, for the row. Two things it has to get right:

- **`--events` goes to a FIFO.** The first attempt wrote it to a file and
  filled a 16 GiB `/tmp` before the sweep reached its second entry — an
  exhaustive `zebra2-minus-15 -m 3` narrates **72.6 M events**, and the file
  reached **7.1 GB** before that first attempt was killed.
  The census keeps sixteen integers per layer; the stream that carried them
  never lands.
- **A run is killed above `--max-rss-mb`, not only above `--timeout`.** Four
  entries have no finite hypothesis space and reach 14 GB at `-m 5`, and this
  sweep deliberately runs `solve -e` on entries that do not declare it.

## 2. The corpus, split three ways

The classifier is `deaths at layer 1`, because that is a question a reader can
put to a new puzzle in 24 ms — `solve -e -m 1` *is* layer 1.

| regime | cells | enterings | what it means |
|---|---:|---:|---|
| **no-search** | 131 | 0 | phase 2 never ran a layer. 79 under `examples/`, 7 `stdlib/`, all **45** of `tests/stdlib/` — three declarations and two facts apiece, decided at root |
| **pruning** | 4 | 11 749 | layer 1 killed something, so there are clauses |
| **barren** | 45 | 2 189 278 | layer 1 entered candidates and killed **none** |

**Four cells prune, and they are the four the engine was tuned on.**

| entry | `alive` at layer 1 | deaths | `next` |
|---|---:|---:|---:|
| `branching/07_lookahead_off.ein` | 204 | 162 — 79 % | 42 |
| `zebra2-hints.ein` | 36 | 23 — 64 % | 0 |
| `zebra2.ein` | 56 | 32 — 57 % | 11 |
| `zebra.ein` | 56 | 31 — 55 % | 13 |

That is the point of the split. [F9](../../followups/f9_e_catalog.md)'s cluster
note rejected the search-optimisation catalogue on the grounds that *"a complete
cardinality-BFS over a connected corpus leaves no purchase for any of them"* —
and every one of those judgements was taken on a puzzle in the left-hand
column. **45 of 49 cells are in the other one.**

The scale matters as much as the sign, and the classifier does not carry it:
`branching/05_mini_zebra` is barren over three candidates and
`saturation/square-unique/terminus` is barren over 153. What the barren regime
costs is `Σₖ C(alive, k)`, so read the `alive` column beside the verdict.

## 3. The powerset, exactly

For 25 of the 49 searching cells, `entered` equals `Σₖ C(alive, k)` on the
nose — not approximately, not asymptotically.

| entry | cap | the sum | entered |
|---|---:|---|---:|
| `saturation/square-unique/terminus.ein` | 3 | `C(153, 1..3)` | **597 057** |
| `features/01_not_and_absent.ein` | 5 | `C(35, 1..5)` | **384 167** |
| `features/05_stdlib_domain_elim.ein` | 5 | `C(35, 1..5)` | **384 167** |
| `saturation/square-unique/corner-house.ein` | 3 | `C(118, 1..3)` | **273 937** |
| `saturation/square-unique/cul-de-sac.ein` | 3 | `C(114, 1..3)` | **247 019** |
| `features/04_open.ein` | 3 | `C(81, 1..3)` | **88 641** |
| `saturation/square-{fwd,bwd}/{floors,houses,meetings}.ein` (×6) | 5 | `C(20, 1..5)` | **21 699** each |
| `syntax/arg-kinds.ein` | 5 | `C(19, 1..5)` | **16 663** |

`features/01_not_and_absent.ein`, layer by layer, is the whole argument in one
table:

| L | alive | frontier | joined | −dead | −clause | entered | deaths | clauses | models |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 35 | 35 | **35** | 0 | 0 | 35 | 0 | 0 | 0 |
| 2 | 35 | 35 | **595** | 0 | 0 | 595 | 0 | 0 | 0 |
| 3 | 35 | 595 | **6 545** | 0 | 0 | 6 545 | 0 | 0 | 0 |
| 4 | 35 | 6 545 | **52 360** | 0 | 0 | 52 360 | 0 | 0 | 0 |
| 5 | 35 | 52 360 | **324 632** | 0 | 0 | 324 632 | 0 | 0 | 0 |

Every `joined` is `C(35, k)`. Every filter column is zero. `alive` is 35 at
every layer because nothing was ever refuted, `next` is `entered` because
nothing was ever solved either, and the run ends because the depth cap says so.

**This is the note's fifth point with the numbers filled in.** At a fixpoint
the state is *incomplete* — a requirement is unmet and a witness must still be
chosen — and the engine has no word for that, so it does the only thing its
vocabulary allows: it proposes every subset of the open arrows in turn and asks
each one whether it is contradictory. None is. There is no such thing as a
"wrong" answer to a question nobody asked.

## 4. `zebra2-minus-15`, all five layers

The phase's case. Layers 1–3 are the census sweep (24.65 s bare, 56 MiB);
layers 4 and 5 come from a **budget probe** — the census row is emitted however
a layer ends, so `-E` stops the run at the first entering of the next layer and
the row still reports what the generation proposed:

```sh
ein solve -e -m 5 -E 205471 --events probe.fifo examples/zebra2-minus-15.ein   # 500 s
```

| L | alive | frontier | joined | −clause | entered | deaths | clauses | models | next |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---:|
| 1 | 96 | 96 | 96 | 0 — **0 %** | 96 | 0 | 0 | 0 | 96 |
| 2 | 96 | 96 | 4 560 | 0 — **0 %** | 4 560 | 1 428 | 1 428 | **28** | 2 911 |
| 3 | 96 | 2 911 | 60 260 | 16 171 — **26.8 %** | 44 089 | 10 149 | 10 149 | **4** | 26 684 |
| 4 | 96 | 26 684 | 245 612 | 88 887 — **36.2 %** | 156 725 | 6 577 | 6 577 | **0** | 117 658 |
| 5 | 96 | 117 658 | 586 982 | 174 376 — **29.7 %** | 412 606 | — | — | — | — |

**`solve -e` is 618 076 enterings — and it finishes in 416 s.** The sum of the
five `entered` columns was computed from the probes *before* the run was
attempted, and the run then entered exactly it:

```
solutions (k)    32
exhausted        false
enterings        618076 (alive=598955 dead_pre=0 dead_post=19121)
layers_explored  5
wall             416374.6 ms
```

So P1d.10's opening line — *"killed at 30 min"*, 2026-08-20 — is a record of
that session and not of this engine. What it reports is **`Ambiguity k=32,
exhausted=false`**: the depth-5 frontier is not empty, so the cap stopped the
search, not the lattice. Seven minutes of work and the completeness claim is
still unproven, which is [S1d.10.5](s1d.10.5_contract.md)'s subject arriving
early. Six readings:

1. **`alive` never shrinks — 96 at every one of the five layers.** Not one of
   the 96 hypothesis facts is ever refuted outright, because a refutation needs
   a *singleton* death and this puzzle has none.
   [S1d.10.3](s1d.10.3_stopping_criterion.md)'s candidate (b) — an exhaustion
   argument over the alive set — is therefore **dead before it is designed**,
   and its own text predicted this: *"a criterion that depends on `alive`
   shrinking is inert in exactly the regime that needs it"*. Corpus-wide,
   `alive` shrinks in **3 of the 46 multi-layer cells**, and all three are in
   the pruning four.
2. **Layer 2's 0 % is structural, not evidence of an inert store.** A layer-1
   death licenses a width-1 clause, and the singleton `(not h)` writeback plus
   the inter-layer retain have already removed that element from the frontier;
   there is nothing left for the clause to catch. The store's first *possible*
   contribution is layer 3.
3. **Where it can contribute, it does, and it grows into the job.** 26.8 %,
   36.2 %, 29.7 %. `C(96, 3)` is 142 880 and layer 3's join proposed 60 260,
   because layer 2's deaths and solutions had already cut the frontier from
   4 560 to 2 911 — so depth ≤ 3 costs 48 745 against a 147 536-commitment
   powerset, **33.0 %**.
4. **The growth is decelerating faster than the layers grow.** `joined` goes
   `47.5× → 13.2× → 4.1× → 2.4×`. That is not the shape of a search that never
   converges; it is the shape of one that is *nearly* done at the depth cap and
   was killed just short.
5. **`d_found` = 3, confirmed from the other side.** Layer 4 entered 156 725
   commitments and added **zero** distinct models. 32 490 of its alive
   enterings reached a state already recorded — 20.7 % of the layer rediscovering
   what depth 3 knew. That is the cost
   [S1d.10.2](s1d.10.2_depth_required.md) is named after, with a number:
   **569 331 of 618 076 enterings — 92.1 % — happen after the last new model**,
   which on the measured run is 6 min 24 s of its 6 min 56 s.
6. **And the deaths dry up as the clauses accumulate.** Deaths per entering
   fall `31.3 % → 23.0 % → 4.2 %` while the store grows `1 428 → 11 577 →
   18 154`. The clauses that filter layer *k* were bought at layer *k−1*, and
   layer 4 buys almost nothing for layer 5 — which is exactly the barren-layer
   mechanism [S1d.10.4](s1d.10.4_conflict_mining.md) proposes to attack,
   arriving at depth rather than at the root.

### 4.1 The depth-10 probe — 2026-08-25, and depths 6–10 add nothing

§4 stops at the default cap because that is where the engine stops. **A run at
`-m 10` says what the next five layers are worth**, and the answer is *nothing*:

| | depth 5 (§4) | depth 10 | |
|---|---:|---:|---:|
| `enterings` | 618 076 | **10 587 736** | **17.1×** |
| `layers_explored` | 5 | **10** | |
| wall, hypothesis search | 416 374.6 ms | **904 643.6 ms** | **2.17×** |
| per entering | 0.674 ms | **0.085 ms** | 7.9× *cheaper* |
| **models** | 32 | **32** | **0 new** |

Provenance and its two caveats, because both matter for what the row can be
used for. The run is
[`examples/zebra2-minus-15-obligations.ein`](../../../examples/zebra2-minus-15-obligations.ein)
— the **obligations twin**, exhaustive, timed, cap raised to 10 — where §4's
row is the hrule original.
[S1d.2.5](../p1d.2_obligations/hypotheses_from_obligations.md) proved the two
identical counter-for-counter through depth 5, so this is the same workload;
**it is also the first evidence the two paths agree past depth 5**, and it
agrees on the thing that matters — the 18 distinct goal-binding tuples the run
reports are exactly the census's.

And the run **does not report `exhausted`**, so this row does not establish it.
Two things say what it would have been. `layers_explored == -m` is
[Q-M1d.6](../open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)'s
truncation signature. And the arithmetic is not close: `alive` is 96 at every
layer (§4 reading 1), so depth 10 covers **0.0001 %** of Σₖ≤₁₀ C(96, k) —
itself 1.6 × 10⁻¹⁴ of the 2⁹⁶ − 1 commitments the lattice contains. Depth is
not the axis on which this search terminates.

Three readings, and the first is the phase's headline:

1. **`d_stop − d_found` is now measured at seven layers, not two.**
   [S1d.10.2](s1d.10.2_depth_required.md) is named after the gap between the
   depth that finds every model (3) and the depth the search stops at (the
   cap). §4 put 92.1 % of the run after the last new model at a cap of 5; at a
   cap of 10 it is **10 538 991 of 10 587 736 — 99.54 %**. The gap is not a
   property of the default; the default was hiding its size.
2. **[T1d.10.2.4](s1d.10.2_depth_required.md) gets its second half.** §7
   answers the corpus question *"does any entry find a model at depth 4 or
   5?"* with **yes** — `saturation/type-exclusivity/*` needs the fifth layer,
   so the default is not dead. This answers the other half for the entry the
   phase is named after: **it finds none at 4, 5, 6, 7, 8, 9 or 10.** The two
   readings coexist and neither is the general rule — which is why
   [S1d.10.3](s1d.10.3_stopping_criterion.md) needs a *criterion* rather than a
   better default.
3. **An entering got 7.9× cheaper, and this run cannot say why.** 0.674 ms at
   depth 5 against 0.085 ms at depth 10 is not noise, and the plausible
   mechanism is that deep enterings die at their first firing
   (`enable_fail_fast_fork`) where §4's layers were **97 % alive** — 598 955 of
   618 076 — and an alive entering saturates to a fixpoint. **That is a
   hypothesis, not a reading**: the log carries no per-layer row, and settling
   it needs the census's `layer` event at `-m 10`, which is
   [S1d.10.1](s1d.10.1_why_it_does_not_finish.md)'s instrument pointed one
   depth further. If it holds, the *cost* of the barren regime is concentrated
   in the shallow layers and the deep ones are cheap noise, which changes what
   [S1d.10.4](s1d.10.4_conflict_mining.md) should attack.

**What it does not show.** Nothing about termination: 15 minutes buys five more
barren layers and leaves the completeness claim exactly where §4 left it. The
32 models were known at depth 3 in 48 745 enterings; everything after is the
search failing to find a 33rd, and still not proving there is none.

## 5. What the clause store is worth

Eight of 49 cells ever have a candidate dropped by a learned clause.

| entry | dropped | of joined | clauses held |
|---|---:|---:|---:|
| `zebra2-minus-15.ein` (`-m 3`) | 16 171 | 24.9 % | 11 577 |
| `branching/07_lookahead_off.ein` | 10 342 | 47.3 % | 570 |
| `branching/06_lookahead_on.ein` | 3 819 | 42.5 % | 399 |
| `branching/08_hypothesis_relation_whitelist.ein` | 497 | 23.2 % | 143 |
| `saturation/type-exclusivity/pets.ein` | 343 | 7.7 % | 12 |
| `branching/02_one_dead_one_alive.ein` | 81 | 11.5 % | 63 |
| `saturation/type-exclusivity/{colors,nationalities}.ein` | 25 | 13.0 % | 4 |

**A layer that kills nothing learns nothing** is confirmed with no exceptions:
every cell with `dropped_nogood > 0` has `nogoods_emitted > 0` at a shallower
layer, and the 35 cells that never learned a clause never dropped a candidate.

The number that decides the phase is the last column against the fifth: 570
clauses buy 47.3 % on `branching/07`, and **11 577 clauses buy 24.9 % on
`zebra2-minus-15`.** Clauses are not scarce there. What is scarce is clauses
*early*, and depth is what makes a clause narrow enough to catch anything.

## 6. One of the two filter arms cannot fire

`dropped_dead` is **0 across all 2 232 330 candidates**, and this is not a
property of the corpus.

`phase2` closes a layer by recomputing `alive`, promoting forced positives, and
then `a_layer.retain(|c| c.iter().all(|e| alive.contains(e)))`; `a_prev` is what
survives. The next layer's join emits `prefix + (s_last, t_last)` out of two
surviving sets, so every element of every candidate is in `alive` — and nothing
touches `alive` between the retain and the join. Layer 1 is not filtered at all.
So the arm is unreachable from the solve loop, and
[`layer_census.rs`](../../../ein.rs/crates/ein-infer/tests/layer_census.rs)
pins it.

`filter_candidate`'s own doc says the check covers *"the single-element
negatives the singleton-death writeback wrote since `a_prev` was computed"* — it
would, but the retain got there first. What this means for the phase: **the
clause store is the only thing that can shrink a layer.** There is no second
mechanism to appeal to, which is why §3's 25 cells walk the whole lattice.

## 7. `d_found` against `d_stop` — a down payment on S1d.10.2

The census reports both depths for free, and the gap is already visible.

| entry | `d_found` | `d_stop` | enterings past `d_found` |
|---|---:|---:|---|
| `branching/06_lookahead_on.ein` | 3 | 5 | 2 463 of 5 173 — **48 %** |
| `branching/08_hypothesis_relation_whitelist.ein` | 3 | 5 | 688 of 1 646 — **42 %** |
| `branching/02_one_dead_one_alive.ein` | 3 | 5 | 203 of 621 — 33 % |
| `zebra.ein` | 1 | 2 | 55 of 111 — 50 % |
| `zebra2.ein` | 1 | 2 | 45 of 101 — 45 % |
| `zebra2-minus-15.ein` (§4) | 3 | 5 (cap) | 569 331 of 618 076 — **92.1 %** |
| …the same file at `-m 10` ([§4.1](#41-the-depth-10-probe--2026-08-25-and-depths-610-add-nothing)) | 3 | 10 (cap) | 10 538 991 of 10 587 736 — **99.5 %** |

The last two rows are the same puzzle and the same `d_found`; only the cap
moves. **The gap is not bounded by anything the search knows** — it is bounded
by `-m`, and the fraction of the run spent past the last new model goes to 1 as
the cap rises. That is the shape of a missing termination argument rather than
of an expensive one.

**And `-m 5` is not a dead default.**
`saturation/type-exclusivity/colors.ein` and `nationalities.ein` find **one
model at depth 4 and four more at depth 5**, which answers T1d.10.2.4 before it
is asked: a puzzle in this corpus does need the fifth layer, so lowering the
default would change answers rather than only timings.

## 8. Where the time goes in this regime — **the lattice is 1.2 %**

T1d.10.1.4. `utils/profile_ein_rs.py solve examples/zebra2-minus-15.ein -e -m 3`
— `perf record --call-graph lbr` on a `--profile profiling` build, 126 548
samples at 4999 Hz, bucketed by the innermost enclosing engine frame. The
profiling binary is within 0.2 % of the release one on the same run, which the
script checks every time.

The stage asked a sharp question: *"whether the under-determined regime is the
same engine costs at a larger count or a different mix — if `generate_layer` and
`filter_candidate` dominate where the determinate profile has the matcher and
the boundary, the optimisation targets are different ones."*

**They do not. It is the same mix.**

The rows are [baseline.md §3](../../../docs/history/m1a_rust/measurements/baseline.md)'s,
unchanged, so the columns are comparable:

| subsystem | `zebra2 -e` | `zebra -e` | **`zebra2-minus-15 -m 3`** |
|---|---:|---:|---:|
| saturate (incl. compile, firing) | 59.7 % | 25.6 % | **40.7 %** |
| match/bind | 29.0 % | 66.9 % | **47.7 %** |
| hypgen/branch | 7.3 % | 5.3 % | **6.1 %** |
| alive/closed | 1.3 % | 1.2 % | **1.2 %** |
| frontend/load | 1.6 % | 0.4 % | 0.0 % |
| contradiction | 0.3 % | 0.4 % | **2.8 %** |
| **canon/key, apriori/elim** | **0.4 %** | **0.1 %** | **1.4 %** — of which `apriori/elim` **1.2 %** |
| fork/copy | 0.0 % | 0.0 % | 0.0 % |
| unattributed | 0.3 % | 0.1 % | 0.0 % |

**The lattice machinery is 1.2 % of the run** — 1.4 % counting `canon/key`
beside it, against **0.4 %** on `zebra2 -e`. So its share did grow, 3.5×, and it
is still nothing: the prefix join that proposed 60 260 candidates and the filter
that walked 11 577 clauses for each one cost together about a quarter of a
second out of 24.6 s. The other 98.8 % is
saturating 44 089 forks — *the same work the determinate puzzles do*, four
hundred times over. `hypgen/branch`'s **cumulative** share is 94.8 %: almost
nothing in this run happens outside a hypothesis.

**That is a result with teeth, and it is a negative one.** Under
[F9](../../followups/f9_e_catalog.md)'s discipline it rules out a whole class of
proposal before anyone writes it: **you cannot optimise your way out of the
barren regime by making the lattice cheaper, because the lattice is not the
cost.** A faster prefix join, a smarter clause index, a better subsumption
check — each is bidding for a share of 1.2 %. The only lever with any room
behind it is **entering fewer commitments**, which is the milestone's thesis
arriving as a profile rather than as an argument.

Two smaller readings:

- **`contradiction` is 2.8 % against 0.3–0.4 % determinate** — a 7–9× larger
  share, and the honest reason is that this run has 19 121 deaths to explain
  rather than 67.
- The top symbol list is the determinate one rearranged:
  `Matcher::walk` 12.6 %, `Saturator::admit_from_boundary` **8.0 %**,
  `FactStore::find` 7.5 %, `Matcher::ground_args` 7.0 %. The NAF boundary that
  [P1a.6](../../../docs/history/m1a_rust/README.md#p1a6--performance)
  spent twelve stages on is still the largest single non-matcher term here too.

## 9. The instrument's own cost — free, measured

§1 says the per-candidate counting is unconditional rather than behind
[`ein_core::counters`](../../../ein.rs/crates/ein-core/src/counters.rs)'s
feature. That is a departure from the discipline, so it is a measurement, taken
against a build of `87f88f6` in a detached worktree — the same engine without
the two increments and with `filter_candidate` returning a `bool`:

| workload | what it stresses | HEAD | + census | Δ |
|---|---|---:|---:|---:|
| `features/01_not_and_absent -e` | 384 167 candidates, **empty** clause store — so the tally is as visible as it can be | 1 648.6 ms | 1 630.3 ms | **−1.11 %** |
| `branching/07_lookahead_off -e` | 21 843 candidates against ≤ 570 clauses | 278.1 ms | 278.4 ms | **+0.12 %** |
| `zebra2-minus-15 -m 3` | 60 260 candidates against 11 577 clauses | 24 634.3 ms | 24 638.1 ms | **+0.02 %** |

Best of 5 / 5 / 3 after a warm-up, pooled sd 15.2 / 2.8 / 123.2 ms — so all
three deltas are **inside the noise**, and the largest is negative. The reason
is structural rather than lucky: `filter_reason` asks exactly the questions
`filter_candidate` asked, in the same order, and the predicate already allocates
a `Vec` and walks the whole clause store per candidate. Two `u64` increments do
not register against that.

**And the goldens moved for exactly one reason.** `corpus_shapes.md5` shifted
210 rows and `events_naf-boundary.jsonl` gained a line. Every moved row kept its
name and its line count, and the same worktree build settles the rest: rendering
the shape's retained event lines from both binaries and stripping `"n": N`
gives **byte-identical output** on `branching/02` (68 lines), `zebra2` (128) and
`branching/07` (116). The digests moved because a new event kind renumbers every
`n` after it — [events.md](../../../docs/kernel/inference/events.md#comparison)
says `n` is a position and not a field — and for nothing else.

## 10. The re-take — 2026-08-26, and what P1d.2 and P1d.3 moved

The census above was taken on 2026-08-24, which is **before**
[S1d.2.4](../p1d.2_obligations/s1d.2.4_obligations_in_the_saturator.md) put an
obligation tally at every fixpoint, before
[S1d.2.5](../p1d.2_obligations/hypotheses_from_obligations.md) put a rung in the
generator, and before [S1d.3.3](../p1d.3_model_sets/the_verdict.md) qualified
the count. Re-taken with the same instrument and the same command when
[P1d.10 was begun](README.md#what-the-reconnaissance-found--2026-08-26):

```sh
utils/layer_census.py --layers --json census.json     # 197 entries, ~9 min
```

| | 2026-08-24 | 2026-08-26 |
|---|---:|---:|
| corpus entries swept | 180 | **197** |
| …that reach the search | 49 | **51** |
| enterings | 2 201 027 | **2 249 873** |
| candidates joined | 2 232 330 | **2 297 347** |
| dropped because an element left `alive` | **0** | **0** |
| dropped by a learned clause | 31 303 — 1.4 % | **47 474 — 2.1 %** |
| cells where layer 1 kills something | 4 | **5** |
| barren cells | 45 | **46** |
| cells whose enterings are exactly `Σₖ C(alive, k)` | 25 — 2 128 512 — 96.7 % | 25 — **2 128 512** — **94.6 %** |
| cells where `alive` ever shrinks | 3 of 46 | **4 of 48** |
| cells where a clause drops a candidate | 8 | **9** |
| cells that never learn a clause | 35 | **35** |

**Every moved row is the two fixtures P1d.2 added and nothing else.**
`examples/zebra2-obligations.ein` and
`examples/zebra2-minus-15-obligations.ein` are the +2 entries that reach the
search, the +1 pruning cell, the +1 `alive`-shrinking cell and the +1
clause-dropping cell; the 25 exact-powerset cells and their 2 128 512 enterings
are **identical to the row**, so 96.7 % → 94.6 % is a larger denominator and
not a smaller numerator. The phase's own entry reproduces to the digit:

| L | alive | frontier | joined | −clause | entered | deaths | clauses | models | next |
|---:|---:|---:|---:|---|---:|---:|---:|---:|---:|
| 1 | 96 | 96 | 96 | 0 — 0 % | 96 | 0 | 0 | 0 | 96 |
| 2 | 96 | 96 | 4 560 | 0 — 0 % | 4 560 | 1 428 | 1 428 | 28 | 2 911 |
| 3 | 96 | 2 911 | 60 260 | 16 171 — 26.8 % | 44 089 | 10 149 | 10 149 | 4 | 26 684 |

— and so does the twin, row for row, which is
[S1d.2.5 §2](../p1d.2_obligations/hypotheses_from_obligations.md)'s
counter-for-counter claim seen from the census's side.

**Only the counters are quoted.** The `ms` and `MiB` columns of this re-take
were taken on a machine that was also running the reconnaissance's other
probes, so they are not comparable with §4's and are not used anywhere. The
counters are deterministic and are.

### 10.1 The number the first census could not report

`owes.declared` did not exist on 2026-08-24 — [S1d.2.6](../p1d.2_obligations/s1d.2.6_verdicts_counters_corpus.md)
added it, and it is the field that separates *a debt paid* from *a debt never
stated*. Crossed against the rows above, by running each searching entry at
`ein solve -m 0 --json-summary`:

| | cells | declare an obligation |
|---|---:|---:|
| reach the search | 51 | **5** — `zebra`, `zebra2`, `zebra2-obligations`, `zebra2-minus-15`, `zebra2-minus-15-obligations` |
| whose enterings are exactly `Σₖ C(alive, k)` | 25 | **0** |
| barren | 46 | 2 |

So the vocabulary M1d built reaches five of the fifty-one cells that search, and
none of the twenty-five that walk a whole powerset. Why that is not a gap is
[the phase README §3](README.md#3-the-vocabulary-reaches-five-of-fifty-one):
those twenty-five are rule demos, and a demo has no requirement to state.

### 10.2 The sweep against the manifest

§1 states, correctly, that this instrument runs `solve -e` on entries that do
not declare it, because *a regime is a property of a puzzle, not of a flag*.
What that costs the headline is worth having as a number, since a reader will
otherwise take 2 249 873 for the corpus's search bill:

| | enterings |
|---|---:|
| the sweep | 2 249 873 |
| under a `solve -e` **`corpus.toml` declares** (33 of the 51) | **408 108 — 18.1 %** |
| …of which `examples/features/01_not_and_absent.ein` | **384 167 — 94.1 %** |
| …of which `examples/zebra2-minus-15.ein` and its twin | **0** — neither declares `solve -e` |

Both readings are true and they answer different questions. *What shape is this
corpus's search?* — a powerset, 25 cells wide. *What does this repository
actually spend?* — 408 108 enterings, of which one feature demo about negation
is 94 %.


## Cross-links

- [S1d.10.1](s1d.10.1_why_it_does_not_finish.md) — the stage
- [`utils/layer_census.py`](../../../utils/layer_census.py) — the instrument
- [events.md § `layer`](../../../docs/kernel/inference/events.md#layer--the-clause-yield-census)
  — the transport
- [`ein-infer/tests/layer_census.rs`](../../../ein.rs/crates/ein-infer/tests/layer_census.rs)
  — the arithmetic, and the inert filter arm
- [F9](../../followups/f9_e_catalog.md) — the rejected search optimisations,
  every one of them judged on a puzzle from §2's left-hand column
- [`ideas.md`](../ideas.md) — the note whose fifth point §3 is a measurement of
