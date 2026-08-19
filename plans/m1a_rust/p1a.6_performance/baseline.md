# P1a.6 baseline — the parity build, measured

**Produced by:** [S1a.6.1](s1a.6.1_profile_baseline.md), 2026-08-18
**Build:** `master` @ `063b1a5`, the P1a.5 parity build — I1 discharged,
T3 472/473 with [D2](../divergences.md) the only cell.
**Machine:** Intel i9-14900HX (8 P-cores + 16 E-cores), Linux 7.1.8,
`powersave` governor, turbo on, everything pinned to **cpu4** (a P-core
sibling) by `utils/bench_env.sh`.

This file is the durable half of the stage: the tables the next six stages
are chosen by, and the artefacts a re-measure diffs against
(`ein.rs/bench-out/*.json`, git-ignored — the tables here are the committed
record, as with `features.md`).

> **The governor could not be pinned.** `scaling_governor` is root-owned on
> this box, so every number below was taken under `powersave` with turbo
> enabled. The compensation is best-of-N plus a printed spread, and the
> spread is small (≤ 2 % on all but one cell); a bench script that asks for
> `sudo` would be worse than a bench script that reports what it ran under.

---

## 1. End-to-end, process against process

`utils/e2e_baseline.py` — `subprocess` + `os.wait4`, best of 3 after a
warm-up, per-child peak RSS. **This is the table the phase's targets are
measured against**, because "end-to-end" in the
[milestone baseline](../README.md#baseline--what-einrs-has-to-beat) means a
process: interpreter start-up, imports, a cold JIT, the run, the print.

| workload | CPython 3.14 | PyPy 3.11 | ein.rs | vs PyPy |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | 5 837.6 ms | 4 938.0 ms | **198.8 ms** | **24.8×** |
| `solve zebra2.ein` | 1 995.3 ms | 2 529.9 ms | **37.6 ms** | 67.3× |
| `solve zebra.ein -e` | 32 686.2 ms | 8 787.4 ms | **585.8 ms** | 15.0× |
| `solve zebra.ein` | 7 651.2 ms | 3 045.1 ms | **120.8 ms** | 25.2× |
| `render rules zebra2.ein` | 300.8 ms | 828.0 ms | **1.1 ms** | 753× |
| `saturate zebra2.ein` | 844.3 ms | 1 410.4 ms | **5.0 ms** | 282× |
| peak RSS (`zebra2 -e`) | 47.0 MB | 223.1 MB | **17.4 MB** | 12.8× |

Spreads: ≤ 2 % everywhere except `solve zebra -e` under PyPy (6.4 %) and
`render` under ein.rs (12.2 % of 1.1 ms, i.e. ±0.1 ms — process start-up).

### Where the milestone's denominators moved

The stage asked for this refresh in as many words: *"a 20× claim measured
against a six-month-old number is not a measurement."* Three of the four
moved.

| number | as recorded | today | why it matters |
|---|---:|---:|---|
| PyPy `solve zebra2 -e` | 4.07 s | **4.94 s** | the ≥ 20× target's denominator; 20× is now ≤ 247 ms, not ≤ 204 ms |
| PyPy `solve zebra -e` | 8.15 s | **8.79 s** | — |
| PyPy parse + load `zebra2` | 0.78 s | **0.43 s** ¶ | the ≥ 50× target's denominator |
| CPython `solve zebra2 -e` | 5.69 s | **5.84 s** | stable |

¶ **0.78 s is not reproducible from its own components.** The milestone's
attribution records PyPy parse at 0.27 s and kb load at 0.37 s (0.64 s), and
today's warm in-process figures are 0.24 s and 0.19 s (0.43 s) — where
`parse` covers nine files and `load` covers `zebra2` alone, so the sum is
two workloads and not one. Recorded rather than reconciled, because the
target is met on every reading: ein.rs parses `zebra2` in 200 µs, *loads* it
in 1.04 ms, and runs a whole `saturate zebra2` **process** — start-up, load,
saturation, print — in 5.0 ms against a ≤ 15 ms target.

**PyPy is slower than CPython on two of the six workloads** — `solve
zebra2` (2 530 vs 1 995 ms) and `saturate` (1 410 vs 844 ms). A short run
never repays the JIT's warm-up, which is exactly the shape of a CLI
invocation and exactly what `utils/bench_baseline.py`'s warm in-process
timing cannot see. Both columns are kept for that reason.

**The two `-e` targets split.** `solve zebra2 -e` is **met** — 198.8 ms
against a ≤ 200 ms target, 24.8× against a PyPy that got faster than the
recorded baseline. `solve zebra -e` is **not** — 585.8 ms against ≤ 400 ms,
1.46× short — and §3 says where its time goes: the matcher, 66.9 % of self
time on that puzzle against 29.0 % on the other.

## 2. The in-process bench set

`utils/bench_baseline.py` (both interpreters) and `cargo bench`, same eight
names. Warm, best-of-N, inside one process — the right shape for comparing
a `parse` against a `parse`, and the wrong one for a headline claim (§1).

| bench | CPython | PyPy | ein.rs | vs CPython / PyPy |
|---|---:|---:|---:|---:|
| `parse` (both puzzles + 7 stdlib modules) | 783.0 ms | 244.4 ms | **780.5 µs** | 1 003× / 313× |
| `parse` (`zebra2` alone) | — | — | **200.2 µs** | — |
| `load` (zebra2: parse + imports + macros + index) | 640.3 ms | 187.2 ms | **1.04 ms** | 616× / 180× |
| `saturate_root` (zebra2) | 85.3 ms | 90.1 ms | **2.76 ms** | 31× / 33× |
| `match_hot` (every plan, saturated root) | 2.2 ms | 5.4 ms | **38.9 µs** | 57× / 139× |
| `boundary` (zebra2) ‡ | 85.8 ms | 66.2 ms | **2.79 ms** | 31× / 24× |
| `fork` + first delta write | 17.3 µs † | — | **257 ns** | 67× |
| `solve_fast` (zebra2) | 1 881.2 ms | 758.2 ms | **35.59 ms** | 53× / 21× |
| `solve_exhaustive` (zebra2) | 5 749.9 ms | 2 299.2 ms | **198.14 ms** | 29× / 12× |

Every ein.rs figure is today's criterion mean (§6), not a number carried
forward — so the whole table is one day's measurement on one machine, which is
the only kind of ratio worth printing.

† `fork` is below `perf_counter` resolution in `bench_baseline.py` (it prints
0.0 ms); the CPython figure is P1a.2's measurement of the same thing.

‡ **The two `boundary` rows were not the same workload**, and putting them
in one table is how that surfaced: `utils/bench_baseline.py` times a
*zebra2* root saturation and the criterion group timed a *zebra* one — the
column above would have read 9.3× and meant nothing. Fixed in the bench set
rather than in a footnote: the group now runs both puzzles (`boundary/zebra`
7.32 ms, `boundary/zebra2` 2.79 ms), and the row above is the comparable
one. The set is **nine cases across eight names** as of this stage.

**`boundary` is the one bench where PyPy beats CPython** (66.2 vs 85.8 ms),
which is a hint on its own: the boundary is a `_watch_stamp` loop over
integers, the shape a tracing JIT is best at, and §7 item 2 shows what that
same loop became in a layered KB.

## 3. Attribution — where the time goes

`utils/profile_ein_rs.py`: `perf record --call-graph lbr` on a
`--profile profiling` binary (release codegen + line tables; identical
timing, re-checked on every run), bucketed into
`utils/profile_solve.py`'s subsystems by the **innermost enclosing engine
frame**, so allocator and data-model leaves land on the engine function
that asked for them — the way cProfile's `tottime` accounts for the
equivalent C-level work.

> **LBR, not frame pointers, and that was measured.** With `--call-graph
> fp`, 57 % of an exhaustive `zebra2`'s samples arrive with no caller:
> glibc is built without frame pointers, so every sample that lands in
> `malloc` loses its stack — i.e. exactly the allocator cost
> [S1a.6.2](s1a.6.2_memory_layout.md) needs attributed. `dwarf,8192`
> truncated at two frames. LBR recovers `malloc ← Vec::from_iter ←
> compile::plan_key ← Engine::compile_for ← Saturator::step` in full and
> leaves **0.3 %** unattributed. Its known weakness is stale history in the
> *outer* frames, which is why the cumulative column is a lower bound and
> the self-time column is not.

### Self time by subsystem

| subsystem | `zebra2 -e` | `zebra -e` | ein.py `zebra2 -e` (for shape) |
|---|---:|---:|---:|
| saturate (incl. compile, firing) | **59.7 %** | 25.6 % | small self, >99 % cumulative |
| match/bind | 29.0 % | **66.9 %** | 46 % |
| hypgen/branch | 7.3 % | 5.3 % | — |
| alive/closed | 1.3 % | 1.2 % | — |
| frontend/load | 1.6 % | 0.4 % | — |
| contradiction | 0.3 % | 0.4 % | — |
| canon/key, apriori/elim | 0.4 % | 0.1 % | — |
| fork/copy | 0.0 % | 0.0 % | 0.01 % |
| unattributed | 0.3 % | 0.1 % | — |

**The two puzzles are two different profiles**, and the phase's stage list
has to serve both: `zebra2 -e` is dominated by saturation-side work
(compile, boundary, enqueue), `zebra -e` by the join itself. The missed
target is `zebra -e`, so the matcher is on the critical path whatever the
other column says.

### Top symbols, `zebra2 -e` (10 298 samples)

| self | symbol |
|---:|---|
| 15.2 % | `[libc.so.6]` — allocator internals, attributed to callers in the table above |
| 11.3 % | `match_::Matcher::unify` |
| 9.3 % | `iter::Chain::fold ⟨ein_core::kb::Kb::n_facts_of⟩` |
| 5.6 % | `match_::Matcher::try_candidate` |
| 5.6 % | `match_::Matcher::walk` |
| 3.6 % | `saturator::Saturator::admit_from_boundary` |
| 3.0 % / 2.9 % | `malloc` / `cfree` |
| 2.2 % | `saturator::Saturator::enqueue_binding` |
| 2.1 % | `compile::Compiler::premise` |
| 2.1 % / 2.0 % | `facts::FactStore::get` / `::args` |
| 1.8 % | `intern::Interner::intern` |
| 1.8 % | `HashMap::contains_key ⟨firing::BindingKey⟩` |

### Top symbols, `zebra -e` (14 803 samples)

| self | symbol |
|---:|---|
| 36.8 % | `match_::Matcher::unify` |
| 11.3 % | `match_::Matcher::try_candidate` |
| 9.0 % | `match_::Matcher::walk` |
| 7.6 % | `[libc.so.6]` |
| 4.5 % | `iter::Chain::fold ⟨ein_core::kb::Kb::n_facts_of⟩` |
| 3.6 % / 3.4 % | `facts::FactStore::get` / `::args` |
| 2.5 % | `btree::map::Iter::next` |
| 2.4 % | `saturator::Saturator::admit_from_boundary` |

### Cumulative (LBR lower bounds), `zebra2 -e`

| ≥ | passes through |
|---:|---|
| 41.4 % | `Saturator::admit_from_boundary` |
| **21.1 %** | `ein_infer::compile` |
| **19.7 %** | `compile::PlanMemo` |
| 12.5 % | `Matcher::run` |
| 9.5 % | `Kb::n_facts_of` |
| 2.6 % | `Interner::intern` |

**Is the boundary still 72 %?** No — it is *at least* 41.4 % and the honest
answer needs an in-engine timer rather than LBR: `examples/engine_cost.rs`
puts it at 80 % of a `zebra` **root saturation**. What has changed is that
it is no longer the only thing worth looking at: **compile is a fifth of the
exhaustive run**, which [design/06](../design/06_saturation.md) § Win A
predicted at 12 % and for which no stage was ever written.

## 4. What the engine *did* — the work counters

`ein_core::counters` (feature-gated, compiled out by default) against
`utils/count_work.py` (runtime wrappers on ein.py, no source edits). Same
field names, so the tables diff row for row.

`zebra2 -e` / `zebra -e`, ein.py → ein.rs:

| counter | `zebra2 -e` py | rs | `zebra -e` py | rs | verdict |
|---|---:|---:|---:|---:|---|
| `unify_slot` | 5 980 322 | 5 980 766 | 60 392 123 | 60 235 905 | +0.007 % / −0.26 % |
| `candidates` | 4 646 381 | 4 609 731 | 29 640 102 | 29 507 263 | −0.79 % / −0.45 % |
| `walk` | 474 446 | 481 851 | 1 092 635 | 1 078 117 | +1.6 % / −1.3 % |
| `plan_compile` | 17 430 | 17 430 | 5 138 | 5 138 | **exact** |
| `fact_insert` | 2 175 | 2 175 | 6 766 | 6 766 | **exact** |
| `guard_query` | 33 113 | 33 113 | 69 317 | 69 317 | **exact** |
| `watch_stamp` | 324 492 | 324 492 | 411 452 | 411 452 | **exact** |
| `watch_stamp_rel` | 644 166 | 644 166 | 820 008 | 820 008 | **exact** |
| `fork` | 101 | 104 | 111 | 114 | +3 |
| `binding_key` | 445 414 | 82 465 | 726 361 | 200 283 | **5.4× / 3.6× fewer** |
| `plan_run` | 13 732 | 83 159 | 4 953 | 113 986 | not comparable |
| `prov_node` | 0 | 0 | 0 | 0 | a solve walks no provenance |
| *(enterings)* | 101 | 101 | 111 | 111 | **exact** |

Read in three groups:

- **The five exact rows are a parity result stronger than T1.** T1 compares
  the counters the engine publishes; these are counters nobody published,
  and the two implementations agree on every one digit for digit —
  including 644 166 per-relation extent counts and 17 430 plan compilations
  on the same run.
- **The three near-exact rows differ only in abandoned work.** ein.py
  materialises `_candidates` as a tuple and ein.rs iterates `facts_with`
  lazily, so a join that breaks after three candidates costs three here and
  the whole bucket there (`candidates_offered`: 6 853 068 on `zebra2 -e`,
  1.49× the candidates actually tried). Where the two abandon differs by a
  candidate or two, and nothing downstream sees it: firings, facts and
  enterings are identical, which is what T2 already proves.
- **Two rows are structural.**
  `binding_key` is lower because ein.rs caches the key on the queue entry
  where ein.py recomputes it at four sites — an optimisation already taken.
  `plan_run` counts per-disjunct in ein.rs and per-plan in ein.py, so it is
  a mapping artefact and is recorded rather than chased.

> **`plan_compile` read 180 against 17 430 on the first attempt, and that
> was the instrument.** `utils/count_work.py` wraps a module attribute, and
> `engine.py` binds `compile_rule` into its own namespace with `from .compile
> import compile_rule` — so the wrapper never saw the caller that does all
> the work, and an apparent **97× gap** was really a 1:1 match. It is
> recorded here because it was nearly published: the number was plausible
> (ein.rs *does* recompile per engine), it agreed with a real finding (§7
> item 1), and it was wrong. What caught it was a second instrument
> disagreeing — the `compile` **event** stream, which both implementations
> emit **17 250** of on that run, in files of 183 231 identical lines.
> `count_work.py` now rebinds every `ein.*` module attribute that holds the
> function and reports how many it replaced under `-v`. It found a second
> unwrapped binding while it was there — `lookahead.py`'s own `_bind_args` —
> which changed no number, because neither puzzle reaches it. That is the
> other half of the lesson: an instrument has to report its own coverage,
> since a wrapper that saw nothing and a call site that ran nothing produce
> the same zero.

**`plan_compile` left the exact group at
[S1a.6.8](s1a.6.8_compile_cache_and_extents.md)** — 17 430 → **305**, against
ein.py's unchanged 17 430 — and a new `extent_probe` row joined the table.
Every other counter above is still bit-identical after that stage;
[§10](#10-after-s1a68--the-same-instruments-re-run) has both.

## 5. Memory

`examples/alloc_cost.rs` — a counting global allocator plus the `Dumper`
hook, so the fork deltas measured are the real forks of a real search.

| cell | allocations | churn | peak live | forks | max KB depth |
|---|---:|---:|---:|---:|---:|
| `zebra2` fast | 418 771 | 22.5 MB | 2.00 MB | 11 | 4 |
| `zebra2 -e` | 2 536 702 | 134.7 MB | 8.34 MB | 101 | **35** |
| `zebra` fast | 446 794 | 44.9 MB | 1.61 MB | 13 | 5 |
| `zebra -e` | 3 136 307 | 299.9 MB | 2.94 MB | 111 | **34** |

Process peak RSS for all four cells in one binary: **9.9–11.7 MB** across
three runs of the *same* binary. That spread — ~19 % — is the allocator's
high-water mark, not the program's, and it is recorded because it sets the
noise floor for every RSS claim in this phase: an "improvement" of 1 MB here
would be indistinguishable from a re-run. (The CLI's 17.4 MB in §1 includes
the renderers and the answer table.)

**Per-fork delta** — the saturated fork's own layer, which is all a fork
owns:

| cell | min | median | p90 | max | mean |
|---|---:|---:|---:|---:|---:|
| `zebra2 -e` facts | 2 | 14 | 43 | 54 | 18 |
| `zebra2 -e` bytes | 0.7 K | **2.1 K** | 8.1 K | 9.0 K | 3.6 K |
| `zebra -e` facts | 14 | 52 | 88 | 109 | 56 |
| `zebra -e` bytes | 1.1 K | **2.9 K** | 7.0 K | 7.6 K | 3.9 K |

**The number [P1a.7](../p1a.7_parallelism/README.md) asked for: a fork
costs ~3.6 KB, worst case 9 KB.** A thousand concurrent searches is 4 MB of
deltas. `--jobs` is not memory-bound and does not need to be sized by RAM
at all — which retires the open sizing question before P1a.7 starts, and
moves the constraint to what design/08 always said it was: determinism.

**134.7 MB of churn over 2.5 M allocations is ~53 bytes per allocation** —
small, short-lived objects on a hot path, which is what the 21 % combined
`malloc` / `cfree` / `[libc]` self time is. [§7](#7-the-top-five-costs)
item 4.

## 6. `cargo bench` — variance, and the acceptance gate

The stability requirement was **< 3 % run-to-run variance on every bench**
before any number from the set is believed. `utils/criterion_table.py` reads
criterion's own `estimates.json` — the console line does not print a
standard deviation — and exits non-zero if any bench misses, so the gate is
checked rather than asserted:

| bench | mean | sd | rsd | 95 % CI |
|---|---:|---:|---:|---|
| `boundary/zebra` | 7.32 ms | 103.2 µs | 1.41 % | [7.30, 7.34] ms |
| `boundary/zebra2` | 2.79 ms | 52.6 µs | 1.89 % | [2.78, 2.80] ms |
| `fork/zebra2` | 257 ns | 4 ns | 1.49 % | [256, 258] ns |
| `load/zebra2` | 1.04 ms | 19.3 µs | 1.86 % | [1.03, 1.04] ms |
| `match_hot/zebra2` | 38.9 µs | 602 ns | 1.55 % | [38.8, 39.0] µs |
| `parse/corpus` | 780.5 µs | 18.7 µs | **2.40 %** | [777.2, 784.5] µs |
| `parse/zebra2` | 200.2 µs | 3.5 µs | 1.76 % | [199.6, 201.0] µs |
| `parse/zebra2_resolve` | 817.0 µs | 11.0 µs | 1.35 % | [815.2, 819.5] µs |
| `saturate_root/zebra2` | 2.76 ms | 18.4 µs | 0.67 % | [2.76, 2.76] ms |
| `solve_exhaustive/zebra2` | 198.14 ms | 2.80 ms | 1.41 % | [196.89, 199.99] ms |
| `solve_fast/zebra2` | 35.59 ms | 202.6 µs | 0.57 % | [35.48, 35.72] ms |

**11 cases, worst relative sd 2.40 %, gate met** — on a `powersave`
governor with the machine in normal use, which is worth knowing before
anyone tries to justify a bench reboot ritual.

`solve_exhaustive` at 198.14 ms agrees with §1's 198.8 ms **process**
measurement to 0.3 %, which is the cross-check that matters: parse and load
are 1.8 ms of that run, so the search really is the whole cost and the two
instruments really are measuring the same thing.

### The acceptance gate

| gate | tests | CPython | PyPy | ein.rs |
|---|---:|---:|---:|---:|
| the three fixtures (`zebra_two_ontologies`, `zebra_three_classes`, `mode_consistency`) | 19 py / 3 rs | — | **36.0 s** | **1.27 s** → **0.62 s** at S1a.6.9 |
| the whole `acceptance/` gate (`./run_tests.sh --acceptance-only`) | 21 | 140.2 s § | **49.3 s** | — |

**The ≤ 5 s target is met at 1.27 s**, 28× against PyPy's 36.0 s on the
same three fixtures — measured on the test binary directly, three runs
within 10 ms of each other, so it is the tests and not `cargo`.

The full-gate row moved again, as [design/README](../design/README.md#measured)'s
‡ note predicted it would: ~91 s at S1.21.8, 43.7 s at P1a.0, **49.3 s**
today. Nothing in ein.py changed and P1a.0 was *yesterday* on this same
machine, so the 13 % is what a single un-repeated process measurement is worth
— which is why every number in this file is best-of-N with its spread printed
next to it. Third recorded value of the same quantity; each one is real and
none of them is *the* number.

§ **The CPython column is 19/21, and the two failures are not engine
results.** `acceptance/test_bench_solve_mode.py` spawns `sys.executable -m
ein.cli`, and the system interpreter has no `ein` installed
(`ModuleNotFoundError`), where `.venv-pypy` has it editable. The remaining
19 pass. Recorded because the number is real and the caveat is what makes it
readable, not to be fixed here.

## 7. The top five costs

Each with the design section that predicted it — or the one that should
have.

### 1. Plan re-compilation — 21.1 % of `zebra2 -e`, and Win A was never built

`Engine::compile_for` misses, and `PlanMemo::intern` re-compiles, once per
engine per (rule, activator) pair: **17 430 compiles** on an exhaustive
`zebra2`, of which 17 250 are the engine misses both implementations emit a
`compile` event for. ein.py does exactly the same number — this is not a
regression against the oracle, it is an **unclaimed 21 %**, and
`engine.rs`'s own module comment describes the design that would claim it —

> *"which is why the **process-wide memo** holds the plans and each engine
> keeps its own ordered list ([design/06](../design/06_saturation.md) §
> Win A)"*

— and `memo: PlanMemo` is a **field of `Engine`**, so every *saturator* gets
a fresh one: one per fork saturation, plus one per `lookahead` probe and one
per `closed` marking. The doc predicted the fix and the code shipped the shape it was
contrasting itself with. Hoisting the memo into a shared, refcounted store
while each engine keeps its own insertion-ordered plan list is
parity-preserving *by construction*: the cache order that reaches the trace
is the per-engine list, which does not move.

`Interner::intern` belongs to this item too, not to a separate one: its
callers are **`Compiler::slot` (42 %) and `Compiler::premise` (33 %)**, so
the interning on the profile's hot list is the compiler's, and a memo hit
does none of it.

**Predicted by:** [design/06](../design/06_saturation.md) § Win A — which
predicted the *count* exactly (17 430 → ~170 distinct pairs) and the saving
at 12 %, where the Rust profile now says 21.1 %.
**Missed by:** nothing — this is an implementation gap, not a design one,
which is why it is first: the cheapest 20 % in the phase, and the design
work is already written.

> **Claimed at [S1a.6.8](s1a.6.8_compile_cache_and_extents.md) T1a.6.8.1**
> (2026-08-18). `plan_compile` **17 430 → 305**, `ein_infer::compile`
> **21.1 % → 2.4 %** cumulative, `solve zebra2 -e` −18.3 % from this half
> alone — and half the run's allocations went with it, which is the part
> item 4 below gets for free. The saving is 1.5× what design/06 estimated
> and the distinct-pair count is 1.8× what it guessed.
> [§10](#10-after-s1a68--the-same-instruments-re-run).

### 2. `Kb::n_facts_of` under the watch stamp — 9.5 %, and it grows with depth

`watch_stamp_into` asks for 644 166 relation extent sizes on `zebra2 -e`.
In ein.py that is `len(fbr.get(rel, ()))` on one flat dict — O(1). In
ein.rs the KB is layered, so it is a hash lookup **per layer**, and the
exhaustive search reaches **depth 35** (§5). The stamp that
[design/06](../design/06_saturation.md) § Boundary introduced to make NAF
re-evaluation cheap is itself 9.5 % of the run — entirely because of the
layered COW model, and growing with the depth the search reaches.

**Predicted by:** nobody. [design/03](../design/03_data_model.md) §5 argues
for O(1) forks and never asks what a *count* costs afterwards; design/06
assumes the stamp is free because it is in ein.py.
**The fix is small:** a per-relation count maintained on insert, `O(1)`
regardless of depth, or a flatten threshold (T1a.6.2.5) that keeps depth
bounded. Both are invisible to every observable.

> **Claimed at [S1a.6.8](s1a.6.8_compile_cache_and_extents.md) T1a.6.8.2**
> (2026-08-18), by the first of the two: `Kb` maintains the count, the fold
> is gone, `n_facts_of` is **9.5 % → 1.2 %** self and off `zebra -e`'s top-20
> entirely. Worth 13.9 % of `zebra2 -e` and **7.3 % of `zebra -e`** — the
> larger share of the run that misses its target. `fork` pays 12 ns for it.
> T1a.6.2.5's flatten threshold is still worth sweeping: the *other* layered
> reads (`layers_rev` under `contains`, `Chain::try_fold` under `facts_of`)
> are 4.2 % of `zebra2 -e` and this fix does not touch them.

### 3. The matcher — 66.9 % of `zebra -e`, 29.0 % of `zebra2 -e`

`unify` + `try_candidate` + `walk` = 57.1 % of `zebra -e`'s self time, and
`zebra -e` is the workload that misses its target by 1.46×. The work
counters say ein.rs performs the *same* 60.2 M slot unifications as ein.py
(−0.26 %), so this is not a search doing more — it is 60 M of a 586 ms run,
about 6 ns each.

**Predicted by:** [design/05](../design/05_matcher.md) §§1, 6 — the
register machine exists because of this line in the Python profile, and it
delivered a 57× `match_hot`. What remains is per-candidate constant factor:
`FactStore::get` + `::args` (7.0 % combined on `zebra -e`) is a double
indirection per candidate, which is exactly T1a.6.2.2's bucket-major
storage, and beta-memories ([S1a.6.3](s1a.6.3_beta_memories.md), F11 D1)
are the structural answer.

### 4. Allocator traffic — 21 % of `zebra2 -e` self time, 2.5 M allocations

`malloc` 3.0 % + `cfree` 2.9 % + `[libc.so.6]` 15.2 %, at ~53 bytes per
allocation. LBR names the callers: `compile::plan_key`'s `Vec<String>`,
`BindingKey`'s boxed register slice, `saturator::Entry`'s drop glue,
`Interner::intern`. Item 1 removes a large share of it for free (a memo hit
allocates nothing); the rest is
[S1a.6.2](s1a.6.2_memory_layout.md) T1a.6.2.3/4.

**Predicted by:** [design/03](../design/03_data_model.md) §10 asks for the
allocation count to be bounded and reported, which is what
`alloc_cost.rs` now does. Not predicted: that `plan_key` would be one of
the top allocators.

### 5. The boundary's ordered re-scan — 2.5 % of `zebra -e`, and 64 % of it cumulatively

`admit_from_boundary` is **64.4 % cumulative on `zebra -e`** (41.4 % on
`zebra2 -e`) and 2.4–3.6 % of self time, and a further **2.5 %** is
`btree::map::Iter::next` — 95 % of whose samples come from
`admit_from_boundary`, i.e. the ordered walk of the `parked` set itself,
before any guard is re-asked. Item 2 is the stamp each parked entry is
tested with; this is the cost of visiting them at all, once per quiescence.

**Predicted by:** [design/06](../design/06_saturation.md) § Win B, which
says the boundary "is not [semi-naive]: a parked candidate whose watch stamp
moved re-runs its whole negative query" — and by
[Q-M1a.17](../open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate),
where Win B's own ≥ 80 % assumption met a measurement and lost. The ordered
container was chosen for determinism ([design/02](../design/02_determinism_and_order.md)),
so the fix is not "use a hash set" but "do not walk what cannot have
changed" — which is the same shape as item 2 and probably the same commit.

## 8. What this chooses for the rest of the phase

| stage | profile says | order |
|---|---|---|
| **[S1a.6.8](s1a.6.8_compile_cache_and_extents.md)** (new) ✅ | items 1 + 2: 21.1 % and 9.5 %, both parity-preserving by construction, both small | **shipped 2026-08-18** — −30.5 % / −7.8 %, [§10](#10-after-s1a68--the-same-instruments-re-run) |
| **[S1a.6.9](s1a.6.9_fork_entry_delta.md)** (new, added after this list) | [§9](#9-the-fork-entry-re-derivation): 95.0 % of `zebra -e` is fork saturation and 94.6 % of that is re-derivation. The measurement and the decision run **second**; the shipping half is gated on [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) and runs last | **second** |
| [S1a.6.2](s1a.6.2_memory_layout.md) Memory layout | item 4 (21 % allocator) and item 3's `FactStore` indirection; T1a.6.2.5 gains a second reason (depth 35), and §9 adds two tasks — a system allocator (T1a.6.2.7) and a per-entering region (T1a.6.2.8) | third |
| [S1a.6.3](s1a.6.3_beta_memories.md) Beta-memories | **gate opens**: 66.9 % of `zebra -e` is the join, and a fork's delta is 3.6 KB, so F11's "a memory copied per fork can lose more than it saves" no longer holds. §9 gives it its target: the *root* memories are the invisible way to remove the re-derivation | fourth |
| [S1a.6.4](s1a.6.4_hypgen_and_lattice.md) Hypgen and lattice | 7.3 % / 5.3 % self — real, smaller than written. **T1a.6.4.1's premise needs its own measurement first**: the interning on the profile's hot list is the *compiler's* (42 % `Compiler::slot`, 33 % `Compiler::premise`), so "18 k interns per `complete()` call" is not what this profile shows | fifth |
| [S1a.6.5](s1a.6.5_frontend.md) Frontend and load | **already met**: `parse zebra2` 200 µs, `load` 1.04 ms, and the whole `saturate zebra2` process 5.0 ms against a ≤ 15 ms target. Reduce to a confirmation + the allocation report its acceptance asks for | sixth, short |
| [S1a.6.6](s1a.6.6_differential_fuzzer.md) Differential fuzzer | unchanged — it guards everything above | throughout |
| [S1a.6.7](s1a.6.7_relever_matrix.md) Re-lever matrix | unchanged | last |

## 9. The fork-entry re-derivation

**Added 2026-08-18, same build and machine**, after the § 7 list was
written. It is not one of the five costs above — it is the *shape* three of
them share, and it is measured here because it is bigger than any of them
individually. [S1a.6.9](s1a.6.9_fork_entry_delta.md) is the stage;
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
is the decision it needs.

`commitment::try_commitment_set` forks the *saturated* root, writes the
commitment's `k ≤ 5` hypothesis facts, and builds a **fresh** `Saturator`:
fresh engine, empty `seen` / `fired` / `parked`, and `delta = None`, which
is a FULL pass. The closure is semi-naive within a saturation
([design/06](../design/06_saturation.md) §4: `pos_index` + `run_seeded`,
D2 + D5) but **not across the fork boundary**, which is where the delta is
smallest and known exactly.

> **Historical from 2026-08-19.** This section measures the *fresh* fork
> saturator, which is what ein.py does and what ein.rs did until
> [S1a.6.9](s1a.6.9_fork_entry_delta.md). It is kept because it is the
> measurement that stage was chosen by, and because it is still what
> `utils/fork_split.py` reports for ein.py or for a `fork-delta` build under
> `EIN_FORK_DELTA=0`. **For the shipping engine, read
> [§11](#11-the-resumed-fork-saturator-measured)**: 9 834 fork firings on
> `zebra2 -e` rather than 38 136, and 0 fork compiles rather than 12 625.

Split at the `enter` events of a `--events-level verbose` run —
**`utils/fork_split.py`**, the T1a.6.9.1 instrument that replaced the inline
script this section was first written with:

| `-e` run | enterings | alive / dead | fork firings | **redundant** | productive | fork enqueues | fork compiles |
|---|---:|---:|---:|---:|---:|---:|---:|
| `zebra2` | 101 | 34 / 67 | 38 136 | **36 442 (95.6 %)** | 1 694 | 81 766 | 12 625 |
| `zebra` | 111 | 40 / 71 | 113 746 | **107 610 (94.6 %)** | 6 136 | 198 763 | 3 552 |

Per entering, and the root for scale. `between` is the inter-layer work —
`compute_alive`, the forced positives and the **root** re-saturations they
run — which belongs to neither column:

| | firings | redundant | productive | enqueues | parks | compiles | quiesces |
|---|---:|---:|---:|---:|---:|---:|---:|
| `zebra2` root | 321 | 100 | 221 | 691 | 835 | 250 | 40 |
| `zebra2` between | 0 | 0 | 0 | 0 | 0 | 4 375 | 0 |
| `zebra2` mean fork | 377.6 | 360.8 | **16.8** | 809.6 | 240.8 | 125.0 | 31.1 |
| `zebra` root | 880 | 470 | 410 | 1 520 | 651 | 64 | 119 |
| `zebra` between | 0 | 0 | 0 | 0 | 0 | 1 312 | 0 |
| `zebra` mean fork | 1 024.7 | 969.5 | **55.3** | 1 790.7 | 208.3 | 32.0 | 107.5 |

> **These are the corrected numbers; the first printing of this table was
> mis-split**, and the instrument exists because the correction is not
> visible by eye. The engine did not change — every firing count in the run
> is identical to the one at `fe62f94` — but two attributions were wrong:
>
> - **`enter` closes the block it describes.** `solve.rs` emits it *after*
>   `try_commitment_set` returns, so treating it as an opener folded the
>   first entering into the root row (`zebra2` root: 810 firings against a
>   true 321) and left a trailing tail that is not an entering at all. The
>   corrected cut is checked against the `enter` event's own `n_firings`,
>   which matches on every block of both runs.
> - **`compile` events between enterings are hypgen's, not a fork's.**
>   `compute_alive` and the pre-branch lookahead compile too: 4 375 of
>   `zebra2`'s 17 250, which is why the fork share is 12 625 (**125 per
>   entering**) rather than 16 875 (~167).
>
> The headline is unmoved — 95.6 % and 94.6 % to the tenth — because both
> errors moved a numerator and its denominator together.

And the enclosing share (`utils/profile_ein_rs.py --cum-of`):

| run | cumulative in `ein_infer::commitment` |
|---|---:|
| `zebra -e` | **95.0 %** |
| `zebra2 -e` | **86.7 %** |

**On `zebra -e` — the one workload that misses its target — 95 % of the run
is fork saturation and 95 % of what fork saturation does is re-deriving the
root's fixpoint, 111 times.** An entering contributes 55 productive
firings and pays for 961 redundant ones.

Three of § 7's five costs are this cost seen from different angles: item 1
(21.1 % re-compilation) is 12 625 of the run's 17 250 `compile` events
happening *inside* forks, 125 per entering; item 3 (the matcher, 66.9 % of
`zebra -e`) is where the re-derivation is actually paid; item 4's
allocation churn is largely its by-product.

### Is the re-derivation load-bearing?

Partly, and **most — but not all — of the part that is stays reachable from
the delta.** `alt` fires when `Kb::record_justification` records a *new*
alternative justification:

| run | `alt` total | at root | in forks | own firing **redundant** | premises include a fork fact | **root-only premises** |
|---|---:|---:|---:|---:|---:|---:|
| `zebra2 -e` | 5 111 | 96 | 5 015 | 5 015 (100 %) | 4 317 | **698** |
| `zebra -e` | **0** | 0 | 0 | 0 | 0 | 0 |

The first printing of this table read "4 335 after a redundant firing, 776
after a productive one", and that split is an artefact of the direction of
attribution: `record_alternative` emits its `alt` lines **before** the `fire`
line of the firing that produced them, so pairing each `alt` with the
*preceding* `fire` reports a productive share that cannot exist. Every `alt`
in the engine comes from a redundant firing, because `record_alternative` is
only reached on the `all_known` path.

The split that does mean something is the last two columns. 4 317 of the 5 015
fork alternatives are recorded by a firing that **reads a fork-local fact** —
a hypothesis or something derived from one — and a delta-seeded pass finds
those by construction. The other **698 read root facts only**: their premises
and their conclusion all pre-date the fork, so a resumed saturator that
inherits `fired` never re-enqueues them. They are not obviously lost either —
a candidate that was *parked* at root and is admitted in the fork has
root-only premises and is still found, because [T1a.6.9.4](s1a.6.9_fork_entry_delta.md)
inherits the parked set — which is exactly why T1a.6.9.2 verifies the
alternatives map rather than arguing about it. On `zebra` the redundant
firings record nothing **at all**, which is why that puzzle shows the cost at
its purest.

### The same boundary, seen by the allocator

[§5](#5-memory) measured a fork's surviving delta at a **3.9 KB** mean
(`zebra -e`). The same run allocates 3 136 307 times for 299.9 MB of churn
across 111 enterings — **≈ 28 000 allocations and ≈ 2.7 MB per entering**
(root's share included, and it is ~5 %). So on the order of **0.15 % of what
a fork allocates outlives it**: registers, trails, binding keys, guard-set
ids, `Entry` boxes, plan-key `Vec<String>`s, compile scratch — all of it
dies at the same instant, and 64 % of enterings die entirely.

That is a region, not a heap, and it is why
[S1a.6.2](s1a.6.2_memory_layout.md) gained T1a.6.2.7 (a system allocator
with per-thread caches) and T1a.6.2.8 (a per-entering arena). Both are
invisible to every observable, which is the opposite of the situation
S1a.6.9 is in.

### Reproducing this section

```sh
python3 utils/fork_split.py                      # both tables, both cells
python3 utils/fork_split.py --json fork.json     # and the artefact

utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 3 \
    --cum-of ein_infer::commitment solve examples/zebra.ein -e
```

`fork_split.py --bin <other-ein>` runs the same split against a second
binary, which is how [T1a.6.9.3](s1a.6.9_fork_entry_delta.md) sizes the
resumed saturator's effect on the narration.

## 10. After S1a.6.8 — the same instruments, re-run

**2026-08-18, `master` @ `d944c4a`, same machine.** The tables above are the
S1a.6.1 baseline and stay as they were: they are what the phase is measured
*against*, and overwriting them would leave the phase with no denominator.
This section is the first re-measure, and every later stage adds its own.

### The four targets

| workload | target | at S1a.6.1 | **at S1a.6.8** | vs PyPy today |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | ≤ 200 ms | 198.8 ms | **138.1 ms** (−30.5 %) ✅ | **37.0×** |
| `solve zebra.ein -e` | ≤ 400 ms | 585.8 ms | **539.9 ms** (−7.8 %) ❌ | 16.2× |
| parse + load `zebra2` | ≤ 15 ms | 1.04 ms | **1.01 ms** ✅ | 185× |
| the acceptance gate (3 fixtures) | ≤ 5 s | 1.27 s | **1.02 s** (−19.7 %) ✅ | 35× |

`zebra -e` is **1.35× short** where it was 1.46×. It is still the target the
phase turns on, and § 9 says where the rest of it is.

### End-to-end, and the two halves separately

`utils/e2e_baseline.py`, best of 7, machine at loadavg 0.4:

| workload | at S1a.6.1 | at S1a.6.8 | change |
|---|---:|---:|---:|
| `solve zebra2 -e` | 198.8 ms | **138.1 ms** | −30.5 % |
| `solve zebra2` | 37.6 ms | **30.3 ms** | −19.4 % |
| `solve zebra -e` | 585.8 ms | **539.9 ms** | −7.8 % |
| `solve zebra` | 120.8 ms | **116.7 ms** | −3.4 % |
| `render rules zebra2` | 1.1 ms | 1.0 ms | — |
| `saturate zebra2` | 5.0 ms | 4.9 ms | — |
| peak RSS (`zebra2 -e`) | 17.4 MB | **17.4 MB** | unchanged |

The stage is two independent changes and they were built separately to keep
them attributable — `taskset -c 4`, best of 4, one series:

| build | `zebra2 -e` | `zebra -e` | `zebra2` | `zebra` |
|---|---:|---:|---:|---:|
| S1a.6.1 (P1a.5 parity build) | 198.8 ms | 585.8 ms | 37.6 ms | 120.8 ms |
| **+ T1a.6.8.1** (per-run plan memo) | 162.4 ms | 585.0 ms | 33.1 ms | 122.4 ms |
| **+ T1a.6.8.2** (O(1) extent count) | **139.8 ms** | **542.2 ms** | **31.2 ms** | **117.7 ms** |

**The two puzzles are moved by different halves**, which is § 7's finding
arriving as a result: the memo is worth 18.3 % on `zebra2 -e` and **0.1 %** on
`zebra -e` (19 plans against 6), and the extent count is worth 7.3 % on
`zebra -e` against 13.9 % on `zebra2 -e`. Either change alone would have
looked like a wash on one of the two.

### `cargo bench`

| bench | at S1a.6.1 | at S1a.6.8 | change |
|---|---:|---:|---:|
| `parse/corpus` | 780.5 µs | 765.0 µs | −2.0 % |
| `parse/zebra2` | 200.2 µs | 197.5 µs | −1.3 % |
| `load/zebra2` | 1.04 ms | 1.014 ms | −2.5 % |
| `saturate_root/zebra2` | 2.76 ms | 2.700 ms | −2.2 % |
| `match_hot/zebra2` | 38.9 µs | 39.3 µs | +1.1 % |
| `boundary/zebra` | 7.32 ms | 7.252 ms | −0.9 % |
| `boundary/zebra2` | 2.79 ms | 2.710 ms | −2.9 % |
| **`fork/zebra2`** | 257 ns | **268.9 ns** | **+4.6 %** |
| `solve_fast/zebra2` | 35.59 ms | **28.37 ms** | −20.3 % |
| `solve_exhaustive/zebra2` | 198.14 ms | **133.10 ms** | −32.8 % |

Two rows are worth more than the two big ones.

**`fork` regressed, and it should have.** T1a.6.8.2 clones a
`relation → u32` map per fork; 12 ns against ~104 forks is 1.2 µs on a run it
takes 43 ms off. Recorded because [rule 3](README.md#rules-for-this-phase)
only works if a regression inside a win is still written down.

**`boundary` barely moved** — −2.9 % and −0.9 % — while the same fix was worth
7.3 % of `zebra -e` end-to-end. The bench saturates a **root**, where
`Kb::depth()` is 1 and the fold it replaced was already O(1). The cost existed
only for a search deep enough to have layers, which is why S1a.6.1's bench set
could not see it and its profile could. A bench set that only measures roots
cannot price a fix to the search.

### Attribution

`utils/profile_ein_rs.py`, self time by subsystem:

| subsystem | `zebra2 -e` before | after | `zebra -e` before | after |
|---|---:|---:|---:|---:|
| saturate | 59.7 % | 44.7 % | 25.5 % | 19.8 % |
| match/bind | 29.0 % | **42.2 %** | 66.9 % | **72.6 %** |
| hypgen/branch | 7.3 % | 8.4 % | 5.1 % | 5.1 % |
| frontend/load | — | 1.8 % | 0.5 % | 0.4 % |

| symbol | `zebra2 -e` before | after |
|---|---:|---:|
| `ein_infer::compile` (cumulative) | **21.1 %** | **2.4 %** |
| `Kb::n_facts_of` (self, via the fold) | **9.5 %** | **1.2 %** |
| `Matcher::unify` (self) | 12.9 % | 15.8 % |
| `[libc.so.6]` + `malloc` + `cfree` | 21.1 % | 17.9 % |

On `zebra -e`, `Kb::n_facts_of` leaves the top-20 entirely and
`ein_infer::compile` is 0.3 % cumulative. **The matcher is now 72.6 % of that
run's self time** — the phase's remaining work is one subsystem, and it is the
one [S1a.6.3](s1a.6.3_beta_memories.md) is for.

### Work counters

Only two rows moved, and one of them is new:

| counter | before | after | note |
|---|---:|---:|---|
| `plan_compile` (`zebra2 -e`) | 17 430 | **305** | **the one counter the two implementations are meant to disagree on** — ein.py still compiles 17 430. 305 is the distinct `(rule, activator)` pair count on that run, reported rather than predicted; design/06 § Win A guessed ~170, and the forks derive activators the root never had |
| `plan_compile` (`zebra -e`) | 5 138 | **242** | |
| `extent_probe` (`zebra2 -e`) | — | 646 184 | new: map probes inside `n_facts_of`, the instrument for its O(1)-in-depth claim. 644 166 of them are the watch stamp's; the other 2 018 are hypgen's and apriori's |

Every other counter is **bit-identical** — `unify_slot`, `unify`,
`candidates`, `walk`, `plan_run`, `binding_key`, `fact_insert`,
`guard_query`, `watch_stamp`, `watch_stamp_rel`, `fork`. Nothing about the
search changed, which is the claim the stage had to support and the reason
the counters exist.

### Memory

| cell | allocations before | after | churn before | after | peak live |
|---|---:|---:|---:|---:|---:|
| `zebra2` fast | 418 771 | **235 582** | 22.5 MB | 16.1 MB | 2.00 MB (=) |
| `zebra2 -e` | 2 536 702 | **1 344 404** | 134.7 MB | 93.1 MB | 8.35 MB (=) |
| `zebra` fast | 446 794 | **382 870** | 44.9 MB | 42.6 MB | 1.61 MB (=) |
| `zebra -e` | 3 136 307 | **2 729 516** | 299.9 MB | 285.0 MB | 2.94 MB (=) |

**Half of `zebra2 -e`'s allocations were the compiler's.** A memo hit
allocates nothing, and § 7 item 4 predicted exactly this — `plan_key`'s
`Vec<String>` and `Interner::intern` under `Compiler::slot` were on its
caller list. Peak live is unchanged to the reported precision and process
peak RSS is 9.7 MB, inside the 9.9–11.7 MB band § 5 recorded as this
machine's noise. The per-fork delta distribution does not move at all: the
saved allocations were transient by construction.

### The gate

**T3 472/473, with [D2](../divergences.md) the only differing cell** — the
same 472/473 the P1a.5 parity build left. The `--events-level verbose` stream
for `solve zebra2.ein -e` is **byte-identical** to the pre-change build,
183 231 lines, which is the check that matters for T1a.6.8.1: a shared memo
must change memo hits and nothing else, and the `compile` event fires on an
*engine* miss.

### Reproducing this section

```sh
utils/bench_env.sh python3 utils/e2e_baseline.py --impl ein.rs --runs 7
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 \
    --cum-of ein_infer::compile solve examples/zebra2.ein -e
cd ein.rs
cargo run --release --features counters -p ein-infer --example counter_cost
cargo run --release -p ein-infer --example alloc_cost
```

## 11. The resumed fork saturator, measured

**[S1a.6.9](s1a.6.9_fork_entry_delta.md), 2026-08-19**, same machine. §9
measured the cost; this measures what removing it buys and what it moves.
**It shipped**: `Saturator::resume` is the path ein.rs takes, and
[Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)
was answered ein.rs-only, with
[D3](../divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it)
recording what that costs.

Every diff below comes from **one binary**: a `--features fork-delta` build
compiles in the way back to the old fresh-fork saturator (`EIN_FORK_DELTA=0`),
so the two arms differ by `Saturator::new` against `Saturator::resume` and by
nothing else. That build is D3's fixture, and `utils/fork_delta_verify.py` is
how the divergence stays measured rather than merely accepted.

### What it removes

`utils/fork_split.py --bin ein.rs/target-fd/release/ein`, with and without
the switch (`EIN_FORK_DELTA=0` is the *fresh* row):

| `-e` run | | fork firings | **redundant** | productive | fork enqueues | fork compiles |
|---|---|---:|---:|---:|---:|---:|
| `zebra2` | fresh | 38 136 | 36 442 (95.6 %) | 1 694 | 81 766 | 12 625 |
| | **resumed** | **9 834** | **8 131 (82.7 %)** | 1 703 | **11 981** | **0** |
| `zebra` | fresh | 113 746 | 107 610 (94.6 %) | 6 136 | 198 763 | 3 552 |
| | **resumed** | **26 656** | **20 520 (77.0 %)** | 6 136 | **30 043** | **0** |

The **productive** column is the check: 6 136 → 6 136 on `zebra`, and +9 on
`zebra2` where fail-fast stops a dying fork at a different firing. The same
facts get derived; what goes is the re-derivation, 74–77 % of the firings and
85 % of the enqueues. Fork compiles go to **0** — §7's item 1 in full, because
a resumed fork inherits root's plan list rather than rebuilding it.

### What it costs, and what it gains

Best of 5, same machine, `target-fd` binary both arms:

| workload | fresh | **resumed** | | vs PyPy |
|---|---:|---:|---:|---:|
| `solve zebra2.ein` | 31.1 ms | **27.1 ms** | 1.15× | 93× |
| `solve zebra2.ein -e` | 137.5 ms | **100.2 ms** | 1.37× | 49× |
| `solve zebra.ein` | 115.1 ms | **97.3 ms** | 1.18× | 31× |
| `solve zebra.ein -e` | 525.6 ms | **392.6 ms** | 1.34× | 22.4× |

**`zebra -e` crosses its ≤ 400 ms target** — the phase's one unmet target,
and the reason S1a.6.9 exists. On the shipping binary it is **394.2 ms** and
`zebra2 -e` is **99.3 ms**.

### The optimisation that was not worth building

77 % of the firings for 34 % of the time is a suspicious ratio, and the
obvious suspect was the snapshot: it is deep-copied per entering — engine,
candidate arena, `seen`, `parked` — where an `Arc`-shared layered one would
not be. That was going to be T1a.6.9.4's main work.

**`perf` says it is 0.6 %.** `alloc::vec::Vec::clone<Entry>` is the only
snapshot symbol on `zebra -e`'s top-20, and the whole `fork/copy` subsystem is
**0.1 % self, 0.1 % cumulative**. What is left is the matcher, and it grew
from 66.9 % of `zebra -e` to **80.5 %** — the fork boundary's share went *to*
the join rather than away, which makes the remaining cost
[S1a.6.3](s1a.6.3_beta_memories.md)'s subject and not this stage's. Building
the layered snapshot would have been the wash
[Rule 3](README.md#rules-for-this-phase) exists to prevent.

| `zebra -e` self time | at S1a.6.8 | at S1a.6.9 |
|---|---:|---:|
| match/bind | 72.6 % | **80.5 %** |
| saturate | — | 13.2 % |
| hypgen/branch | — | 4.4 % |
| fork/copy (the snapshot) | — | **0.1 %** |

### What it moves — the narration

| stream | `zebra2 -e` | `zebra -e` |
|---|---:|---:|
| `--events --events-level verbose` (T2) | 183 231 → 68 670 (**−62.5 %**) | 405 367 → 97 723 (**−75.9 %**) |
| `--events` at `normal` | 146 689 → 60 439 (−58.8 %) | 297 287 → 76 733 (−74.2 %) |
| `solve --trace` steps under the hypothesis | 561 → **240** | — |
| `solve --trace` steps, root section ‖ | 0 → **321** | — |

**T2 moves at `normal` too**, not only at `verbose` — Q-M1a.18 was written
expecting the verbose-only loss. A redundant firing is not emitted at
`normal`, but the ~1 790 `enqueue` lines per entering that produce it are, and
those go with it.

‖ **The trace gained a section rather than losing steps**, and that was not
optional. Rendering only the solution node's firings used to pick up root's
whole closure by accident — every fork re-derived it — so a resumed fork
dropped `symmetric` out of the proof entirely, because `symmetric` closes
`next-to` at root and nowhere else. `--trace` now opens with **"Before any
assumption — 321 steps"**, then `Assuming …`, then the 240 the hypothesis
adds, numbered as one sequence. Same 24 rules as before, in the order
[`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
tells it. The rendered before/after is
[`fork_delta_trace.md`](fork_delta_trace.md).

### What it costs the parity harness

Against ein.py, which keeps the fresh saturator:

| tier | before | after | cells D3 costs |
|---|---|---|---:|
| T3 (artefacts) | 472 / 473 | **465 / 473** | **7** |
| T2 (event stream) | 239 / 240 ‡ | **142 / 240** | **97** |
| T1 (`summary.json`) | — | unchanged | **0** |
| T0 (the verdict) | — | unchanged | **0** |

‡ the one cell in each *before* column is
[D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
which predates this.

The seven T3 cells are exactly the seven corpus entries that declare a
`solve --trace` or a `solve --dump-states` run — every other artefact a solve
writes is unmoved, stdout and `summary.json` included.
[S1a.6.10](s1a.6.10_parity_contract.md) is the stage that teaches the harness
to compare what a fork *derived* instead of the whole firing list, and
[§12](#12-the-parity-contract-relaxed-and-re-measured) is what happened when
it did: **472 / 473 and 239 / 240**, D2 the only cell in either. The numbers
above are what D3 cost in between.

### What it does **not** move — verified, not argued

`utils/fork_delta_verify.py`: one binary, two arms, every `solve`-family run
of every `positive` / `stdlib` corpus entry — plus `-p`, `-P`, `-f` and
`-e -p`, the answer-printing forms the corpus does not declare and the twelve
unsat entries' answers therefore live in — comparing artefacts that are not
firing lists.

| | fail-fast **on** (shipping) | fail-fast **off** |
|---|---:|---:|
| runs / entries | 370 / 65 | 370 / 65 |
| enterings compared fact by fact | **3 228 853** | **3 170 461** |
| entering count | **0** | **0** |
| entering `kind` | **0** | **0** |
| **alive** fork fixpoint, fact by fact | **0** | **0** |
| stdout (verdict, `k`, models, bindings, core) | **0** | **0** |
| `summary.json` — **T0 + T1**, all 85 fields ° | **0** | **0** |
| `--dump-states` tree ¶ | **0** | **0** |
| unsat core, per entering | 110 | **0** |
| dead fork's *partial* state ‡ | 2 067 | **0** |

° every solve cell gets `--json-summary`, exactly as the conformance harness
does, and only the clock is normalised out of it. This is the row the whole
decision rests on: the two engines publish the same verdict, the same `k`, the
same enterings, layers, saturations, merges and learned clauses. **They run
the same search and reach the same answer**; they narrate different amounts of
it.

¶ with the wall-clock fields and the firing counts normalised out — those are
the narration, tabulated above.
‡ `enable_fail_fast_fork` stops a *dying* fork at the firing that kills it, so
two firing orders leave two different partial states **by design**. Both
columns agree that the *fixpoint* is identical: with fail-fast off, every
fork — alive and dead — reaches the same fact set, the same `kind` and the
same core.

All 110 core moves are `dead-post` and all disappear with fail-fast off, so
they are the fail-fast prefix rather than a different conflict: on `zebra2`'s
entering 68 the commitment is `{(color-loc Green House-5), (color-loc Red
House-5)}` and the core is one of its two elements — each a correct
single-element core, and which one you get is which clash you reach first.

### What it *does* move, and §9 did not predict — the proof structure

This is the finding. Over the same sweep, on artefacts that are **not** firing
lists:

| | corpus, ff on | corpus, ff off | `zebra2 -e` | `zebra -e` |
|---|---:|---:|---:|---:|
| facts whose **primary** justification changed | 267 529 | 238 211 | 17 | 198 |
| facts whose alternatives changed **membership** | 80 | 168 | 2 | 76 |
| facts whose alternatives changed **order** | 183 378 | 189 661 | 71 | 956 |

The two puzzle columns are fail-fast off, so they are the *fixpoint's* proof
graph rather than a fail-fast prefix's. Six corpus entries carry nine tenths
of it — `examples/features/02_star_in_identifiers.ein` and the five
`examples/saturation/square-*` fixtures, which are transitive-symmetric
closures where every derived fact has many equally valid derivations. On the
zebra puzzles it is 17 and 198 facts.

S1a.6.9 § What is *not* at risk argued that the alternative justifications
survive because "a duplicate of a root-recorded justification is already
rejected". That is true and it is not the mechanism that matters. The one that
does is **admission order at the NAF boundary**:

- a fresh fork rediscovers root's parked candidates in one FULL pass and
  numbers them in *plan* order, interleaved with its own;
- a resumed fork inherits them with root's tiebreakers, so they all sort
  *before* the fork's own at equal priority.

At most one candidate is admitted per boundary round, so a different order is
a different admission sequence, and a fact derivable two ways — the engine is
full of dual rules, `functional-negative`/`injective-negative`,
`domain-elimination`/`range-elimination`, `total`/`surjective` — gets a
different **first** derivation. First derivation wins, so the primary moves;
and the alternatives list, which `record_justification` keeps sorted by
premise count with ties in arrival order, permutes under it.

**This cannot be designed away.** Matching a fresh pass's numbering requires
running a fresh pass, which is the thing being removed. 4 of `zebra2 -e`'s 34
alive enterings are affected; its **solution node is not**, so the rendered
trace's proof is unchanged and only its step list is shorter. That is luck,
not a property.

### The near-miss

`ein.py/tests/trace/test_idea08_acceptance.py::test_zebra2_fires_walkthrough_rules`
asserts that the solution's firing list exhibits the nine rules
[`zebra_walkthrough.md`](../../../docs/kernel/inference/zebra_walkthrough.md)
narrates, and it is what caught this. With the resumed saturator and the
renderer untouched, the solution node's trace covered **12 distinct rules
instead of 24** and `symmetric` was not among them.

The trace was not wrong; it was *incomplete in a way it had not been before*,
and the reason is worth writing down because it is the general shape of this
whole stage: **`--trace` was getting root's proof for free, by accident.** It
renders one node's firings, and every fork re-derived root's closure into
them. Take the re-derivation away and the accident stops paying.

The fix is the root section above, and it is better than what it replaced —
the givens and the hypothesis are now distinguishable, which they never were.
[T1a.6.11.2](s1a.6.11_fixture_goldens.md) ports the assertion to ein.rs, so
the next change to either half meets the same alarm on the engine that
ships.

### Reproducing this section

```sh
cd ein.rs && cargo build --release --features fork-delta --target-dir target-fd

EIN_FORK_DELTA=1 python3 utils/fork_split.py --bin ein.rs/target-fd/release/ein
python3 utils/fork_delta_verify.py --json ein.rs/bench-out/fork-delta.json
python3 utils/fork_delta_verify.py --no-fail-fast
python3 utils/fork_delta_verify.py -k zebra2.ein --with-trace
```

## 12. The parity contract, relaxed and re-measured

**[S1a.6.10](s1a.6.10_parity_contract.md) + [S1a.6.11](s1a.6.11_fixture_goldens.md),
2026-08-19.** §11 measured what the resumed fork saturator costs the harness;
this is what it costs after the harness was taught the rule.

| tier | before S1a.6.9 | after S1a.6.9 (§11) | **after S1a.6.10** |
|---|---|---|---|
| T3 — artefacts, 473 cells | 472 ‡ | 465 | **472** ‡ |
| T2 — the event stream, 240 cells | 239 ‡ | 142 | **239** ‡ |
| T1 — `summary.json` | — | unchanged | **unchanged** |
| T0 — the verdict | — | unchanged | **unchanged** |

‡ [D2](../divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers),
which predates all of this and is the only differing cell in either tier.

### The cut, chosen by measurement rather than by argument

The T2 comparison could have been narrowed six ways. Each was run over the
**same 240 captured logs** before one was written down — the corpus is the
experiment, not the illustration:

| the derivation is … | cells agreeing |
|---|---:|
| the whole stream (the contract before this) | 142 / 240 |
| the ordered non-redundant firings | 142 / 240 |
| … also eliding `compile` | 213 / 240 |
| … as an ordered `(rule, premises, derived)` | 214 / 240 |
| … as a **multiset** of `(rule, premises, derived)`, per `enter`-delimited segment, `dead-post` excluded | 232 / 240 |
| **… as a multiset of derived facts + the set of rules, same segmentation** | **239 / 240** |

Three things the plan did not predict, and each is why the row above it is not
the answer:

1. **The ordered productive subsequence is not identical.** S1a.6.9's 6 136 →
   6 136 is a *count*; the order moves for the same reason the primary
   justification does. 26 cells still differ under an ordered comparison.
2. **`compile` moves.** A plan-memo *miss* is emitted once per enqueue pass
   that needs the rule, so a fresh fork that re-derives root's closure misses
   where a resumed one does not: **244 against 128** on
   `examples/branching/02_one_dead_one_alive.ein`'s plain `solve`. The
   *distinct* compiles are identical, rule for rule and activator for
   activator.
3. **A dying fork's derivation has to leave the comparison outright.** Every
   mismatch left after the multiset cut was a `dead-post` segment.

### The two controls

A relaxation is only a decision if both directions are checked.

**Negative — does it still catch a real loss?**
[`utils/mutant_ein.py`](../../../utils/mutant_ein.py) runs the *shipping*
binary and deletes one event from the log it wrote:

| `EIN_MUTANT` | what it deletes | T2 must | measured, over `--filter branching` (70 comparable cells) |
|---|---|---|---|
| `productive` | the first `fire` with `redundant = false` | **report** | **68 / 68** cells where the deletion applied; exit 1 |
| `redundant` | the first `fire` with `redundant = true` | pass | 70 / 70, exit 0 |
| `enqueue` | the first `enqueue` | pass | 70 / 70, exit 0 |

The other two of the 70 have no productive firing to delete
(`14_lookahead_unjudgeable :: saturate` emits five events and none of them is
a firing), so the mutation is a no-op and the cell rightly agrees. **The first
run of this control was not 68/68 — it was 66, and the two escapes were the
finding**: `solve -L` on `04_two_levels` and `05_mini_zebra`, where the first
entering is a lookahead probe that dies, so root's saturation shared a
`dead-post` segment and was skipped with it. That is what put the hypgen
boundary in `split`. What still escapes is only what the rule says will: a
derivation lost inside a *dying* fork.

**Positive — is the old contract still available?** The determinism sweep,
ein.py against itself under `PYTHONHASHSEED=0` / `=42`, run with `--strict`:
**473 / 473, zero differences**, 673 s of engine time. That is the run that
found hazards H1 and H4, and one engine against itself has no excuse to
narrate differently — `.github/workflows/nightly.yml` passes `--strict` for
exactly that reason.

### What replaced the bytes

[S1a.6.11](s1a.6.11_fixture_goldens.md): twelve ein.rs goldens, 2 188 lines —
five real solves' traces, two `slice` cones, a fork's own `enterings/` dump
with the timeline's firing counts, the snapshot projection, and three event
streams that between them contain every class the relaxed T2 elides.
`./run_tests.sh` gained a **Phase 3** (`cargo test --workspace`) so the repo's
one documented gate runs both engines: **1 506 + 21 + 302** green.

### Reproducing this section

```sh
cd ein.rs && cargo build --release

# the two tiers, relaxed (the shipping contract)
for T in T3 T2; do
  ./target/release/ein-conformance run --tier $T \
      --impl-a "../.venv-pypy/bin/python -m ein.cli" --impl-b ./target/release/ein
done

# the determinism sweep, unrelaxed
./target/release/ein-conformance run --tier T3 --strict \
    --impl-a "../.venv-pypy/bin/python -m ein.cli" \
    --impl-b "../.venv-pypy/bin/python -m ein.cli" \
    --env-a PYTHONHASHSEED=0 --env-b PYTHONHASHSEED=42

# the negative control, from the repo root
for M in productive redundant enqueue; do
  EIN_MUTANT=$M ein.rs/target/release/ein-conformance run --tier T2 \
      --filter branching \
      --impl-a "$PWD/.venv-pypy/bin/python -m ein.cli" \
      --impl-b "python3 $PWD/utils/mutant_ein.py $PWD/ein.rs/target/release/ein"
done
```

## 13. S1a.6.2 — the layout stage, and the profile it starts from

**2026-08-19, `master` @ `66f24d5`, same machine.** [Rule
6](README.md#rules-for-this-phase) says a stage begins by re-running
[S1a.6.1](s1a.6.1_profile_baseline.md)'s instruments rather than by trusting
the last stage's table, and this time it changed the stage: three of
[S1a.6.2](s1a.6.2_memory_layout.md)'s eight tasks were written against a
profile in which the allocator was 21 % and the compiler was 21 %, and
[S1a.6.8](s1a.6.8_compile_cache_and_extents.md) plus
[S1a.6.9](s1a.6.9_fork_entry_delta.md) have since removed **half of every
allocation the engine makes**.

### Before any change — the same instruments, re-run

| instrument | `zebra2 -e` | `zebra -e` |
|---|---:|---:|
| end-to-end, best of 7 | 98.3 ms | 395.7 ms |
| allocations | 880 053 | 1 674 387 |
| churn | 62.8 MB | 127.6 MB |
| **bytes per allocation** | **71.4** | **76.2** |
| peak live | 2.65 MB | 3.27 MB |
| allocator self time | **20.0 %** | **9.4 %** |
| match/bind self time | 52.9 % | **80.1 %** |

Against [§10](#10-after-s1a68--the-same-instruments-re-run)'s figures the
allocation count is down **35 %** and **39 %**, the churn **33 %** and **55 %**,
and `zebra2 -e`'s peak live 8.35 → **2.65 MB** — a resumed fork holds no
re-derivation. What did *not* fall with them is the mean allocation size: ~53
bytes at S1a.6.1, **71–76** now, because what S1a.6.8 removed was the
compiler's small ones. The 21 % of self time [§7](#7-the-top-five-costs) item 4
attributed to glibc `malloc` is **20.0 %** on `zebra2 -e` and only **9.4 %** on
`zebra -e`, whose remaining cost is 80.1 % one subsystem.

The work counters, for the first time since S1a.6.9 moved them — this is what
the two engines now do *differently*, and the row that matters is that they
still agree on the answer:

| counter | `zebra2` fast | `zebra2 -e` | `zebra` fast | `zebra -e` |
|---|---:|---:|---:|---:|
| `unify_slot` | 1 727 543 | 5 933 579 | 13 936 098 | **51 452 037** |
| `unify` | 1 635 187 | 5 617 846 | 13 625 664 | 50 213 778 |
| `candidates` | 1 344 197 | 4 570 900 | 6 827 109 | 25 160 149 |
| `walk` | 106 598 | 355 843 | 118 708 | 530 405 |
| `plan_run` | 20 422 | 123 254 | 19 471 | 104 409 |
| `binding_key` | 2 077 | 12 694 | 4 890 | 31 563 |
| `plan_compile` | 175 | 305 | 87 | 242 |
| `fact_insert` | 641 | 2 184 | 1 248 | 6 766 |
| `guard_query` | 9 978 | 30 691 | 8 827 | 29 865 |
| `watch_stamp` | 36 943 | 204 158 | 41 040 | 248 043 |
| `extent_probe` | 73 987 | 408 133 | 82 994 | 501 225 |
| `fork` | 13 | 104 | 15 | 114 |

`zebra -e`'s 60.2 M slot unifications at S1a.6.1 are **51.5 M** — S1a.6.9
removed 14 % of the join's work along with the re-derivation — and `zebra2
-e`'s `extent_probe` is 646 184 → 408 133 for the same reason. `plan_compile`
is the one counter the two implementations are meant to disagree on
([§10](#10-after-s1a68--the-same-instruments-re-run)).

### T1a.6.2.7 — the global allocator, measured three ways

Four binaries, one series, `utils/e2e_baseline.py --bin` (which this task
added, so two builds can be compared without moving one of them aside):

| workload | system | `mimalloc` | `jemalloc` | **`snmalloc`** |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | 98.3 ms | 84.4 ms | 87.9 ms | **82.7 ms (−15.9 %)** |
| `solve zebra2.ein` | 24.9 ms | 22.9 ms | 23.2 ms | **22.5 ms (−9.6 %)** |
| `solve zebra.ein -e` | 395.7 ms | **365.7 ms** | 377.7 ms | 366.2 ms (−7.5 %) |
| `solve zebra.ein` | 97.0 ms | **93.5 ms** | 95.0 ms | **93.5 ms (−3.6 %)** |
| `render rules zebra2` ¶ | **1.1 ms** | 1.3 ms | 1.3 ms | 1.6 ms (+45 %) |
| `saturate zebra2` | 4.9 ms | 4.7 ms | **4.4 ms** | 4.6 ms |
| **peak RSS, `zebra -e`** | **17.3 MB** | 24.5 MB (**+42 %**) | **17.3 MB** | 18.5 MB (+6.9 %) |
| peak RSS, `zebra2 -e` | 17.3 MB | 22.5 MB | 17.3 MB | **17.3 MB** |
| binary | 3 459 720 B | +4.9 % | +16.6 % | **+3.2 %** |

Re-run at 9 samples the next series over, the four `solve` cells reproduce
within 0.6 ms.

**snmalloc ships.** It is the fastest on three of the four `solve` cells and
within 0.5 ms on the fourth; `mimalloc` matches it and then costs **7.2 MB of
peak RSS**, which the stage's acceptance names as a thing that may not get
worse; `jemalloc` keeps the RSS and returns a third less of the win, and its
own README gates it on `cfg(not(target_env = "msvc"))`, which
[P1a.9](../p1a.9_bindings_release/README.md) ships binaries for.

¶ **The one regression, and it is start-up.** `render rules` is 1.1 ms of
which almost all is process start-up, and snmalloc's arena set-up costs
**0.5 ms** of it — measured at 21 samples, spread 4.5 %, so it is real and not
noise. Every workload with work in it repays that in the first millisecond;
`saturate zebra2`, at 4.9 ms the shortest one that saturates anything, is
already ahead.

### The same change, in process

`cargo bench`, both arms out of one tree — the bench target declares the
binary's allocator, and `-p ein-conformance --no-default-features` is the
system-allocator arm:

| bench | system | **snmalloc** | change |
|---|---:|---:|---:|
| `parse/corpus` | 790.8 µs | 749.9 µs | −5.2 % |
| `parse/zebra2` | 204.1 µs | 189.8 µs | −7.0 % |
| `parse/zebra2_resolve` | 824.6 µs | 755.1 µs | −8.4 % |
| `load/zebra2` | 1.041 ms | 910.8 µs | −12.5 % |
| `saturate_root/zebra2` | 2.692 ms | 2.10 ms | **−22.0 %** |
| **`match_hot/zebra2`** | 38.33 µs | 38.1 µs | **−0.6 %** |
| `boundary/zebra` | 7.082 ms | 6.69 ms | −5.5 % |
| `boundary/zebra2` | 2.702 ms | 2.10 ms | **−22.3 %** |
| **`fork/zebra2`** | 270.4 ns | 302 ns | **+11.7 %** |
| `solve_fast/zebra2` | 22.90 ms | 20.12 ms | −12.1 % |
| `solve_exhaustive/zebra2` | 95.36 ms | 79.89 ms | −16.2 % |

**`match_hot` is the control and it did not move.** It runs every plan over an
already-saturated root and allocates nothing on the path it times; an allocator
that changed it would be changing something else. That −0.6 % is what "this
measurement is about allocation" looks like when it is checked rather than
asserted.

**`fork` regressed 11.7 %**, as it did at T1a.6.8.2 and for a different reason:
a fork's first allocations touch fresh size classes, and snmalloc's slow path
is slower than glibc's fastbins. 32 ns × 114 forks is 3.6 µs on a run that got
29 ms faster. Recorded because [rule 3](README.md#rules-for-this-phase) only
works if a regression inside a win is still written down.

The variance gate: **11 benches, worst relative sd 2.83 %**, under the 3 %
bar. Two earlier runs of the same set put a *different* bench over it each time
(`match_hot` 3.68 %, then `load` 4.69 % and `parse/corpus` 3.61 %) while the
machine was still settling from a build — the gate is a machine-state check as
much as a bench check, and it is worth re-running before believing a failure.

### Where the time went instead

`utils/profile_ein_rs.py --repeat 10`, self time by subsystem:

| | `zebra2 -e` before | after | `zebra -e` before | after |
|---|---:|---:|---:|---:|
| **allocator** ‡ | **20.0 %** | **9.0 %** | **9.4 %** | **3.0 %** |
| match/bind | 52.9 % | **61.4 %** | 80.1 % | **86.5 %** |
| saturate | 33.8 % | 28.3 % | 13.6 % | 10.0 % |
| hypgen/branch | 8.5 % | 4.6 % | 4.3 % | 1.7 % |

‡ `[libc.so.6]` + `malloc` + `cfree` + `__rdl_alloc`/`__rdl_dealloc` before;
`sn_rust_alloc` + `sn_rust_dealloc` + the residual `[libc.so.6]` after.

Every subsystem's share *fell* except the matcher's, which is the shape a
smaller allocator bill has: the profiler charges allocation to whoever asked
for the memory, so saturate and hypgen were carrying most of it. `zebra -e` is
now **86.5 % match/bind** and its top five symbols are `unify` 49.3 %,
`try_candidate` 14.8 %, `walk` 9.3 %, `FactStore::get` 5.0 % and
`FactStore::args` 4.9 % — 83.3 % of the run in five functions, four of which
are one loop and the fifth is the two-load indirection T1a.6.2.2 and T1a.6.2.6
are about.

### The profiling binary was not the shipping binary, for one hour

The first profile taken after the allocator landed reported a `zebra -e` at
**550.1 ms** where `release` ran 367.7 ms, and put seven snmalloc size-class
helpers on the top-20 — functions an optimised build inlines.

`cmake`-rs picks `CMAKE_BUILD_TYPE` from the `DEBUG` and `OPT_LEVEL` that cargo
passes each build script, so `[profile.profiling] debug = 1` — a line whose
whole purpose is to add line tables without touching codegen — built the
vendored C++ allocator as `RelWithDebInfo` with its own assertions on. The fix
is two package-scoped profile overrides
([`ein.rs/Cargo.toml`](../../../ein.rs/Cargo.toml)), and the profiling binary
is back to **+0.3 %** of release.

Worth the paragraph because of what caught it: the release-vs-profiling line
`utils/profile_ein_rs.py` prints on every run, added at S1a.6.1 on the argument
that *"a profile taken on a binary that runs at a different speed from the
shipped one is measuring a different program"*. It had printed ±0.3 % for a
month. A vendored C dependency is the first thing in this port that could make
it lie, and it did so within an hour of arriving.

### What it does not move

- **Allocation counts and the per-fork delta distribution are identical.**
  `examples/alloc_cost.rs` counts through its own `GlobalAlloc` around
  `System`, so it measures the program rather than the allocator, and 880 053 /
  1 674 387 are the same numbers as before the change. A move there would have
  meant something else changed.
- **Every work counter is unchanged** — the `counters` build is `ein-infer`'s
  example, which links no allocator at all.
- **T3 472/473, [D2](../divergences.md) the only differing cell**, and
  `cargo test --workspace` green including `tests/match_alloc.rs`, the
  inner-loop allocation-count test the stage's notes ask for after every task.
  Nothing outside `#[cfg(test)]` orders, hashes or compares by address, so an
  allocator cannot reach an observable — but the gate ran anyway, as the
  standard.
- **The acceptance gate is 0.58 s** and was never going to move: ein.rs's test
  binaries link no allocator either, which is the honest place to leave them —
  an embedder chooses its own.

### Reproducing this section

```sh
# the bake-off — one binary per allocator, one series
for A in mimalloc jemalloc snmalloc; do
  cargo build --release -p ein-cli --features $A --target-dir ein.rs/target-alloc-$A
done
cargo build --release -p ein-cli --no-default-features --target-dir ein.rs/target-alloc-system
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7 \
    --bin system=ein.rs/target-alloc-system/release/ein \
    --bin snmalloc=ein.rs/target/release/ein

# the in-process arms
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml \
    -p ein-conformance --no-default-features
python3 utils/criterion_table.py --max-rsd 3

# the attribution, and the release-vs-profiling line that has to stay ±1 %
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 solve examples/zebra.ein -e
```

## Reproducing all of it

Every line from the repo root, every measurement through the fingerprint:

```sh
utils/bench_env.sh --report                    # the machine state, nothing else
mkdir -p ein.rs/bench-out                      # git-ignored artefact root

# §1 processes, §2 the Python halves of the bench set
utils/bench_env.sh python3 utils/e2e_baseline.py --json ein.rs/bench-out/e2e.json
utils/bench_env.sh python3 utils/bench_baseline.py --json ein.rs/bench-out/py-cpython.json
utils/bench_env.sh .venv-pypy/bin/python utils/bench_baseline.py \
    --json ein.rs/bench-out/py-pypy.json

# §3 attribution (add `--callers SUBSTR` for a caller breakdown)
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 \
    --cum-of ein_infer::compile --json ein.rs/bench-out/prof-zebra2-e.json \
    solve examples/zebra2.ein -e

# §4 work counters, both sides
utils/bench_env.sh python3 utils/count_work.py -v --json ein.rs/bench-out/work-py.json
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example counter_cost

# §5 memory
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example alloc_cost

# §9 the fork-entry split — re-run at the end of every stage in the phase
python3 utils/fork_split.py --json ein.rs/bench-out/fork-split.json

# §11 the resumed fork saturator — a build of its own, off on every ship path
cargo build --release --manifest-path ein.rs/Cargo.toml \
    --features fork-delta --target-dir ein.rs/target-fd
python3 utils/fork_delta_verify.py --json ein.rs/bench-out/fork-delta.json

# §6 the bench set and its variance gate (§13 adds the system-allocator arm:
# the same benches with `-p ein-conformance --no-default-features`)
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
python3 utils/criterion_table.py --max-rsd 3 --json ein.rs/bench-out/criterion.json

# §12 the parity contract — the two tiers, the determinism sweep, the control
for T in T3 T2; do
  ein.rs/target/release/ein-conformance run --tier $T \
      --impl-a "python3 -m ein.cli" --impl-b ein.rs/target/release/ein
done
ein.rs/target/release/ein-conformance run --tier T3 --strict \
    --impl-a "python3 -m ein.cli" --impl-b "python3 -m ein.cli" \
    --env-a PYTHONHASHSEED=0 --env-b PYTHONHASHSEED=42
for M in productive redundant enqueue; do
  EIN_MUTANT=$M ein.rs/target/release/ein-conformance run --tier T2 \
      --filter branching --impl-a "python3 -m ein.cli" \
      --impl-b "python3 $PWD/utils/mutant_ein.py $PWD/ein.rs/target/release/ein"
done

# the gate this all has to leave green
./run_tests.sh
```
