# Engine feature × config matrix

Which `SolverConfig` knobs are load-bearing for solving `zebra2`, with
measured impact, **in both implementations**. The companion to the
*definitional* config table in
[`docs/api/inference.md`](../../api/inference.md) (what each knob does) and
the engine narrative in
[`architecture_and_algorithms.md`](architecture_and_algorithms.md) (how
each feature works).

> **Audience: engine contributors / advanced authors.** Most puzzle
> authors only need the takeaway below.

## Takeaway

**On `zebra2`, the shipped fast path is robust — keep the defaults and
don't worry about these knobs.** With `stop_after=1` (the default solve),
disabling *any single* lever still finds the correct unique answer, in 1.3 s
under ein.py and 6 ms under ein.rs. The levers earn their keep in
**exhaustive** search (proving uniqueness / unsatisfiability), where two
matter:

- **`enable_singleton_writeback` is the largest lever in either engine, and
  what its absence costs is engine-relative.** Without it, exhaustive
  `zebra2` explores **3 831** commitments instead of 101. ein.py does not
  finish that inside a 90 s budget; **ein.rs finishes it in 1.53 s — 56.6×
  its own baseline.** The lever's job is identical; only the wall it puts you
  into is a property of the engine. Keep it on.
- **`enable_fail_fast_fork` is the one plain speed knob**, and it is worth
  *more* now than when it shipped (S1.9.E23): **2.4× on ein.py and 7.1× on
  ein.rs** exhaustive, 1.3× / 1.8× on the fast path. It changes nothing about
  *what* is found — same verdict, same 101 enterings, same 67 deaths — so its
  whole effect is price per branch. The ratio grew because everything around
  it got cheaper: what it removes is the saturation of forks that are already
  dead, and after [P1a.6](../../../plans/m1a_rust/p1a.6_performance/README.md)
  that is **86 %** of an exhaustive run without it.
- Every other lever is **1.0×** in ein.rs — exactly, not approximately — and
  two are inert on this puzzle by construction (`enable_forced_positive` never
  fires; `enable_symmetric_mirror` has a transparent rule fallback). The one
  exception is `lattice_order="score-sum"`, which is 1.2× because it explores
  33 more commitments (134 vs 101).

**No single lever is *correctness*-load-bearing on `zebra2`**: every
flag-off run that terminated returned the identical solution, in both
engines, and the two engines agree on the verdict, `k`, the goal bindings and
twelve counters in **every** cell of this page. **That sentence is about
`zebra2`, and 2026-08-20 is when it stopped generalising**: on
`examples/branching/06_lookahead_on.ein` and
`examples/lattice/02_genuine_3set_death.ein`, turning
`enable_pre_branch_lookahead` off changes the verdict from `Ambiguity` to
`Contradiction` — because the lookahead filters the very generator that
`complete()` asks. See [the deeper-puzzle section](#the-lookahead-on-a-deeper-puzzle--and-the-lever-that-is-not-a-prune).

## Method

Measured by [`utils/feature_matrix.py`](../../../utils/feature_matrix.py)
(re-run to regenerate; it writes the raw per-cell artifact to
`utils/feature_matrix_results.json`, which is untracked — the tables below
are the committed record).

Each cell is one **fresh process per run** solving `examples/zebra2.ein` with
one lever flipped off the puzzle's own all-on configuration, in two modes:

- **fast** — `stop_after=1` (the shipped default; stops at the first
  complete model), 30 s budget.
- **exhaustive** — `stop_after=None` (explores the whole commitment
  lattice; a disabled prune shows its full blow-up), 90 s budget.

Four things are worth knowing about how the numbers are taken, because three
of them were wrong once:

1. **A lever reaches the engine through the IR, not the CLI.** `ein solve`
   exposes five of these ten knobs as flags; `(config …)` exposes all of them,
   and both loaders keep the *last* block in the file. So a cell is the puzzle
   plus one generated `(config …)` block holding the puzzle's own resolved
   configuration with one key changed — the same bytes handed to both engines,
   and each run reads its own `--json-summary` back to check that the lever it
   names is the one that moved.
2. **The reported time is the engine's own solve** — root saturation +
   hypothesis search, read from `--timing` — which is the quantity the
   in-process harness timed around `solve()`, so this page stays comparable to
   the 2026-08-17 column it replaces. Best of 5 (zebra2) or 3.
3. **The runs go round-robin over the cells.** Measured cell-by-cell, the
   baseline — the divisor of every ratio here — runs first and reads ~20 %
   fast on this machine.
4. **The `control` row is a byte-identical copy of the baseline, measured
   last**, and it is how each column states its own resolution. It reads
   **1.2× under ein.py** on the exhaustive run and **1.0× under ein.rs**: in
   the Python column nothing below ~1.2× is a measurement, and in the Rust
   column 1.0× means 1.0×.

A cell exceeding its budget returns an `Aborted` verdict — the
"won't-finish-if-off" sentinel (`∞`). Counts are `MonotonicStats` enterings;
`×base` is the engine's own solve time against its own baseline.

*Provenance: measured 2026-08-20 on `ein` at `42c99d9`, both engines, same
machine, same run. ein.py under PyPy 7.3.23 (`.venv-pypy`); ein.rs
`target/release` (snmalloc), the P1a.6 build after
[S1a.6.12](../../../plans/m1a_rust/p1a.6_performance/s1a.6.12_boundary_and_snapshot.md).
Intel i9-14900HX, pinned to one P-core by `utils/bench_env.sh`, `powersave`
governor — read the **factors**, not the absolute seconds, and read them
against the `control` row.*

## Fast path (`stop_after=1`) — robust

> **Every `ein.py s` column on this page is a frozen constant.** The runner
> that produced it drove two engines and there is one; only the `ein.rs`
> columns can be refreshed. Full statement, and what that does to a row's two
> halves over time: [§ Refresh](#refresh).

Every lever-off run matches the baseline: **Solution, k=1, the correct
answer, 11 enterings**. Both engines agree on all of it.

| lever off | enterings | ein.py s | ×base | ein.rs ms | ×base |
|---|---:|---:|---:|---:|---:|
| *(baseline — all on)* | 11 | 1.30 | 1.0× | 6 | 1.0× |
| `enable_fail_fast_fork` | 11 | 1.66 | **1.3×** | 11 | **1.8×** |
| `enable_singleton_writeback` | 11 | 1.31 | 1.0× | 6 | 1.0× |
| `enable_symmetric_mirror` | 11 | 1.28 | 1.0× | 6 | 1.0× |
| `enable_forced_positive` | 11 | 1.28 | 1.0× | 6 | 1.0× |
| `enable_path_nogoods` | 11 | 1.26 | 1.0× | 6 | 1.0× |
| `hypgen_scoring="most-constrained"` | 11 | 1.21 | 0.9× | 6 | 1.0× |
| `lattice_order="score-sum"` | 13 | 1.20 | 0.9× | 6 | 1.0× |
| `enable_pre_branch_lookahead` | 11 | 1.16 | 0.9× | 6 | 1.0× |
| `enable_lookahead_kill_cache` | 11 | 1.14 | 0.9× | 6 | 1.0× |
| *(control — the baseline again, last)* | 11 | 1.29 | 1.0× | 6 | 1.0× |

The fast path enters 2 dead forks before it stops, so `enable_fail_fast_fork`
is the only lever with anything to save — and in ein.rs it is the only row
that is not exactly the baseline.

## Exhaustive (`stop_after=None`) — where the levers bite

Baseline: Solution, k=1, **101 enterings (67 dead)** — 3.18 s (ein.py) /
27 ms (ein.rs).

| lever off | verdict | enterings | ein.py s | ×base | ein.rs ms | ×base |
|---|---|---:|---:|---:|---:|---:|
| `enable_singleton_writeback` | **Aborted** (ein.py) / Solution | 3 358 † / **3 831** | **≥90 (∞)** | **∞** | **1 527** | **56.6×** |
| `enable_fail_fast_fork` | Solution | 101 | 7.68 | **2.4×** | **193** | **7.1×** |
| `lattice_order="score-sum"` | Solution | **134** | 3.10 | 1.0× | 33 | **1.2×** |
| `enable_lookahead_kill_cache` | Solution | 101 | 3.44 | 1.1× | 27 | 1.0× |
| `enable_path_nogoods` | Solution | 101 | 3.42 | 1.1× | 27 | 1.0× |
| `enable_forced_positive` | Solution | 101 | 3.37 | 1.1× | 27 | 1.0× |
| `hypgen_scoring="most-constrained"` | Solution | 101 | 3.33 | 1.0× | 27 | 1.0× |
| `enable_symmetric_mirror` | Solution | 101 | 3.14 | 1.0× | 27 | 1.0× |
| `enable_pre_branch_lookahead` | Solution | **111** | 3.14 | 1.0× | 27 | 1.0× |
| *(control — the baseline again, last)* | Solution | 101 | 3.85 | **1.2×** | 27 | 1.0× |

† ein.py's 3 358 is where its 90 s budget cut the search, not a total; ein.rs
runs the same search to the end at **3 831**, which is what the 2026-08-17
table could only record as `3336+`.

**Read the control row before any other.** Four rows in the ein.py column sit
between 1.0× and 1.1×, and the control — the same puzzle, the same
configuration, a different filename — sits at 1.2×. The Python column
therefore says one thing: *`enable_singleton_writeback` and
`enable_fail_fast_fork` matter, and nothing else here is measurable.* The
ein.rs column, whose control is 1.0× and whose cells are within 5 %, says
that plus one more thing: `score-sum` costs 1.2×, and it is the 33 extra
commitments rather than a price per branch.

## Per-lever notes

- **`enable_singleton_writeback`** — caching a refuted singleton's `(not h)`
  at root lets later layers drop `h` in O(1). Without it the exhaustive
  search re-derives those refutations and the commitment count explodes,
  **101 → 3 831 enterings (38×)**. ein.rs pays 56.6× for those; ein.py does
  not finish. The single knob a uniqueness-proving author must keep on.
- **`enable_fail_fast_fork`** — stops a fork's saturation at the firing that
  makes it inconsistent instead of running to quiescence and only then
  scanning. Unique among these levers in changing *nothing* about the
  search: identical enterings, deaths, solutions and clauses, because the
  KB is append-only and a fork inconsistent at firing *n* is inconsistent at
  the fixpoint. **Its ratio grew as the engine got faster** — 1.9× (ein.py,
  2026-08-17) → 2.4× (ein.py, today, and the two are not distinguishable at
  this column's 1.2× resolution) → **7.1× (ein.rs)** — because what it removes
  is a fixed amount of dead-fork saturation and everything else shrank around
  it. On ein.rs it is now the largest lever on the *fast* path too (1.8×).
- **`enable_symmetric_mirror`** — the native `__symmetric__` arg-swap is a
  *fast-path over* the stdlib `symmetric` rule. `zebra2` imports that rule
  (`std.algebra`), so disabling the mirror falls back to it transparently —
  same answer, same cost here (1.0× in both engines). The mirror's benefit
  shows only on puzzles where the matcher cost of the rule dominates.
- **`enable_forced_positive`** — `zebra2` records `forced_positives = 0`
  with it on, so the puzzle never triggers a forced-positive cascade;
  disabling it is a no-op here. Expected to matter on puzzles with
  backbone singletons.
- **`lattice_order="score-sum"`** — the only lever that changes *what is
  explored* without changing the answer: 134 commitments instead of 101, and
  in ein.rs exactly the 1.2× that implies. The default `lex` order is a
  canonical-tuple sort; `score-sum` orders by the hypothesis scores, which on
  this puzzle finds the same model after 33 more dead ends.

## The same matrix on `zebra` — where two levers change sign

`zebra2` is the puzzle this page has always measured, and it is the shallow
one: five projections of five values each, a human solution that never
branches past depth 1. `zebra` encodes the same problem over **one** generic
`co-located` relation spanning 30 attribute values, which is why it is the
corpus's stress case for hypothesis filtering — and two levers read
differently there. Best of 3, same day, same machine, same method.

**Exhaustive** — baseline 111 enterings (71 dead), 7.39 s / 46 ms:

| lever off | enterings | ein.py s | ×base | ein.rs ms | ×base |
|---|---:|---:|---:|---:|---:|
| `enable_singleton_writeback` | 1 277 † / **3 834** | **≥90 (∞)** | **∞** | **2 014** | **43.8×** |
| `enable_fail_fast_fork` | 111 | 21.21 | **2.9×** | **322** | **7.0×** |
| `enable_pre_branch_lookahead` | **134** | 7.91 | **1.1×** | 49 | **1.1×** |
| `enable_lookahead_kill_cache` | 111 | 7.28 | 1.0× | 46 | 1.0× |
| `enable_path_nogoods` | 111 | 7.35 | 1.0× | 46 | 1.0× |
| `enable_symmetric_mirror` | 111 | 7.33 | 1.0× | 46 | 1.0× |
| `enable_forced_positive` | 111 | 7.34 | 1.0× | 45 | 1.0× |
| `hypgen_scoring="most-constrained"` | 111 | 7.37 | 1.0× | 46 | 1.0× |
| `lattice_order="score-sum"` | **62** | **4.80** | **0.6×** | **28** | **0.6×** |
| *(control)* | 111 | 7.46 | 1.0× | 46 | 1.0× |

**Fast** — baseline 13 enterings (3 dead), 2.04 s / 7 ms. Every row is the
control (1.0×/1.1×) except two: `enable_fail_fast_fork` at 1.3× / **2.6×**,
and `enable_singleton_writeback` at 1.4× / **1.9×** — on `zebra` the
writeback is load-bearing on the *fast* path too, at **24 enterings instead
of 13**, where on `zebra2` it was free until the search went exhaustive.

† again the point where ein.py's budget cut the search, against ein.rs's
completed 3 834.

### Two findings, and neither is a default change *yet*

1. **`enable_pre_branch_lookahead` earns its keep here.** On `zebra2` it is a
   wash (111 enterings without it, 1.0× in both engines); on `zebra` it prunes
   **23 of 134** commitments and pays for itself — turning it off is 1.1×
   *slower* in both engines. The 2026-08-17 table's "measures slightly
   negative, a shape to re-measure on a deeper puzzle" is answered: it was
   inside the noise, and on the deeper encoding the sign is positive. Both
   `zebra` and `zebra2` set it explicitly, so no default moves.
2. **`lattice_order="score-sum"` is 0.6× on `zebra` and 1.2× on `zebra2`**,
   and in both engines to the digit — 62 enterings against 111, and 134
   against 101. That is a **candidate default change with a real number
   behind it and a real counter-example beside it**, so it is recorded here
   and left as a decision rather than applied: the two puzzles disagree, `lex`
   is the order every trace and golden in the corpus was recorded under, and
   the lever costs nothing to set per puzzle. What would settle it is the
   blind-enumerator corpus — the cells `zebra`/`zebra2` never reach — measured
   the same way.

## The lookahead on a deeper puzzle — and the lever that is not a prune

`enable_pre_branch_lookahead` was the one row the 2026-08-17 table flagged as
"a shape to re-measure on a deeper puzzle": it read 0.9× on `zebra2` because
it pays a one-step rule simulation per candidate to avoid forks that fail-fast
had already made cheap. Two corpus fixtures are the deeper puzzle, and both
say something the shallow one could not.

**`examples/branching/06_lookahead_on.ein`** — five colours, five houses, four
anchored; the blind enumerator, not an `(hrule …)`. Its sibling `07` is the
same bytes with the lever off, so this is the corpus's own A/B. Best of 3:

| | verdict | enterings | ein.py s | ×base | ein.rs ms | ×base |
|---|---|---:|---:|---:|---:|---:|
| **fast**, lookahead on | Solution k=1 | 67 | 0.41 | 1.0× | 3.6 | 1.0× |
| **fast**, lookahead off | **Contradiction k=0** | **11 501** | 7.92 | **19.2×** | 278 | **77×** |
| **exhaustive**, on | Ambiguity k=22 | 5 173 | 14.63 | 1.0× | 199 | 1.0× |
| **exhaustive**, off | **Contradiction k=0** | **11 501** | 7.97 | 0.5× | 277 | 1.4× |

> **The `ein.rs` column was re-taken 2026-08-22 and the two lookahead-*off*
> cells moved 3.2×** — 896 → 278 and 890 → 277. Not a change to the lookahead:
> M1a [T1a.7.2.0](../../../plans/m1a_rust/p1a.7_parallelism/s1a.7.2_parallel_enterings.md#task-t1a720--the-layer-stack-coalesced-at-the-barrier)
> coalesces root's layer stack at the search's layer barrier, and with the
> lever off this fixture makes 162 mid-layer writebacks, so all 11 501 of its
> forks were walking a 164-layer root. The enterings, the verdicts and the
> counts are unchanged; only the wall clock moved, which is what says the
> lookahead's *finding* is a count and not a timing. The `ein.py` column is a
> frozen constant — that engine left the tree at
> [S1a.10.5](../../../plans/m1a_rust/p1a.10_single_implementation/README.md) —
> so its ×base ratios are the ones taken with it. All four `ein.rs` cells are
> process wall, best of 3, pinned to one P-core; the two lookahead-*on* cells
> are unchanged within noise and differ from the earlier reading only by the
> process overhead this instrument includes.

**`examples/lattice/02_genuine_3set_death.ein`** — the depth-3 fixture, five
forms of puzzle, too small to time (2 ms of process, 0 ms of engine). The
counts say the same thing: **Ambiguity k=3 over 6 enterings** with the
lookahead on, **Contradiction k=0 over 7** with it off. Every other lever on
that fixture is the baseline exactly, in both engines.

### The lookahead is not (only) a prune

Turning a *pruning* aid off should cost time and change nothing else. Here it
changes the **verdict**, identically in both engines, and the reason is in the
definition rather than in the search:

```text
complete(kb)  ≡  hypgen::generate(kb) proposes nothing        # ein-infer/hypgen.rs
```

Generation yields "the candidates that are neither asserted nor
refuted **nor immediately doomed (lookahead)**". So a candidate the one-step
simulation kills is *decided* when the lever is on and *open* when it is off —
and a KB whose only remaining candidates are doomed is a **solution node** in
the first case and not a model at all in the second. With the lever off,
`branching/06` reports `Contradiction` on a puzzle that has 22 models.

Three things follow, and none of them is a bug in the port — both engines
agree to the digit on every cell above:

1. **`enable_pre_branch_lookahead` is correctness-load-bearing on some
   puzzles**, which is the one claim this page has always made in the
   negative. It stays true as written for `zebra2`; it is false in general.
2. The 0.9× that made it look like a bad trade was **inside the method's own
   noise** (the control row now shows what that noise is). On the puzzles
   where it has anything to prune it is worth 1.1× (`zebra`), 4.5× (an
   exhaustive `branching/06`) and **448×** (the fast path of the same file,
   in ein.rs, where the pruned run is 2 ms and the unpruned one 896).
3. It is not confirmable by `enable_lookahead_kill_cache`: with the cache off
   the verdicts are unchanged (`branching/06 -e` is still Ambiguity k=22, at
   5 192 enterings instead of 5 173). The write-back is an optimisation; the
   *filter* is the semantics.

Whether a performance lever should decide what counts as a complete model is a
design question, not a measurement, and it is parked as
[F4 Q40](../../../plans/followups/f4_cross_cutting.md).

## Refresh

These numbers drift as the engine evolves. Regenerate with

```sh
utils/bench_env.sh python3 utils/feature_matrix.py --runs 5
```

and update the provenance line. `--puzzle P` runs the matrix on another
puzzle, `--cells SUBSTR` on a subset (the `baseline` and `control` rows always
run), `$EIN_BIN` names a build other than `ein.rs/target/release/ein`. The
*definitional* knob list lives in
[`docs/api/inference.md`](../../api/inference.md); add a row there for any new
flag.

> **Only the ein.rs column can be refreshed.** The runner drove both engines
> as processes and cross-checked them cell by cell until
> [S1a.10.4](../../../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md);
> the `ein.py s` columns below — and the `control` row's `1.2× under ein.py`,
> which is what states that column's resolution — are **frozen** at the
> 2026-08-20 measurement. A re-run rewrites the ein.rs figures beside them and
> leaves the ein.py ones exactly as they are, so the two halves of a row will
> drift apart with time. That is the intended reading: one is a record, the
> other is a measurement.

## History — the same table, before the port

Kept because the arc is the interesting part, and labelled because the method
changed with it.

**2026-08-17, ein.py only, PyPy, in-process, single run, cell by cell**
(`ein` at `b17e1f5` + the S1.9.E23 fail-fast fork saturation). Fast path:
every lever 1.0–1.3× of a 1.24 s baseline, `enable_pre_branch_lookahead`
0.9×. Exhaustive, against a **3.92 s / 101-entering** baseline:
`enable_singleton_writeback` **Aborted at ≥90 s with 3336+ enterings**,
`enable_fail_fast_fork` **1.9×**, `lattice_order="score-sum"` 1.0× at 134
enterings, `enable_pre_branch_lookahead` **0.9× at 111 enterings** — "10 extra
deaths, cheaper than the lookahead that prevents them" — and everything else
0.9–1.0×.

Two of that table's conclusions did not survive being re-measured against a
control: the 0.9× rows were inside the method's own noise, and the lookahead
is a **wash** on `zebra2` in both engines rather than a slight negative. The
two conclusions that did survive are the two levers named in the takeaway.

**2026-08-18, ein.rs at [P1a.4](../../../plans/m1a_rust/p1a.4_search_layer/README.md)**,
in-process, before the phase optimised anything: every entering count
reproduced exactly, `enable_singleton_writeback` still the one runaway lever —
and the runaway cell *finished*, at 3 831 enterings in 11.3 s. It is 1.53 s
today.
