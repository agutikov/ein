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
| the three fixtures (`zebra_two_ontologies`, `zebra_three_classes`, `mode_consistency`) | 19 py / 3 rs | — | **36.0 s** | **1.27 s** |
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
| **[S1a.6.8](s1a.6.8_compile_cache_and_extents.md)** (new) | items 1 + 2: 21.1 % and 9.5 %, both parity-preserving by construction, both small | **first** |
| [S1a.6.2](s1a.6.2_memory_layout.md) Memory layout | item 4 (21 % allocator) and item 3's `FactStore` indirection; T1a.6.2.5 gains a second reason (depth 35) | second |
| [S1a.6.3](s1a.6.3_beta_memories.md) Beta-memories | **gate opens**: 66.9 % of `zebra -e` is the join, and a fork's delta is 3.6 KB, so F11's "a memory copied per fork can lose more than it saves" no longer holds | third |
| [S1a.6.4](s1a.6.4_hypgen_and_lattice.md) Hypgen and lattice | 7.3 % / 5.3 % self — real, smaller than written. **T1a.6.4.1's premise needs its own measurement first**: the interning on the profile's hot list is the *compiler's* (42 % `Compiler::slot`, 33 % `Compiler::premise`), so "18 k interns per `complete()` call" is not what this profile shows | fourth |
| [S1a.6.5](s1a.6.5_frontend.md) Frontend and load | **already met**: `parse zebra2` 200 µs, `load` 1.04 ms, and the whole `saturate zebra2` process 5.0 ms against a ≤ 15 ms target. Reduce to a confirmation + the allocation report its acceptance asks for | fifth, short |
| [S1a.6.6](s1a.6.6_differential_fuzzer.md) Differential fuzzer | unchanged — it guards everything above | throughout |
| [S1a.6.7](s1a.6.7_relever_matrix.md) Re-lever matrix | unchanged | last |

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

# §6 the bench set and its variance gate
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
python3 utils/criterion_table.py --max-rsd 3 --json ein.rs/bench-out/criterion.json

# the gate this all has to leave green
ein.rs/target/release/ein-conformance run --tier T3 \
    --impl-a "python3 -m ein.cli" --impl-b ein.rs/target/release/ein
```
