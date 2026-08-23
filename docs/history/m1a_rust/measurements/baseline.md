# P1a.6 baseline — the parity build, measured

**Produced by:** [S1a.6.1](../README.md#s1a61--fresh-profile-and-bench-baseline), 2026-08-18
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

> **What can still be re-measured, since [S1a.10.4](../README.md#s1a104--utils-re-aimed-at-one-engine).**
> Half of this file is a two-column comparison and one of the columns has no
> instrument any more.
>
> | | |
> |---|---|
> | **live** | every ein.rs figure. `utils/e2e_baseline.py` (processes, and `--bin` for two builds of one engine), `cargo bench` + `utils/criterion_table.py`, `utils/profile_ein_rs.py`, `ein-infer`'s `counter_cost` / `alloc_cost` / `layout_shape` / `hypgen_calls` / `frontend_cost` examples, `utils/fork_split.py`, `utils/fork_delta_verify.py`, `utils/feature_matrix.py`, `utils/bench_env.sh` |
> | **frozen** | every **CPython** and **PyPy** figure, and every ratio with one in the denominator — including the milestone's `24.8×` / `165×` and §4's whole ein.py column. Their instruments (`utils/bench_baseline.py`, `utils/count_work.py`, `e2e_baseline.py`'s two interpreter rows) left with the engine they measured |
> | **gone** | every `ein-conformance` invocation below, and `utils/mutant_ein.py`. The tier counts (`T3 472/473`, `T2 239/240`) are a record of a gate that was green on the day, not a command. The event cut's control is `ein-infer/tests/event_cut_control.rs` now ([S1a.10.3](../README.md#s1a103--the-corpus-without-a-second-engine)) |
>
> The per-section *"Reproducing this section"* blocks are left as they were
> written — they say what produced the numbers above them, which is the point
> of a record. **[§ Reproducing all of it](#reproducing-all-of-it) at the end
> of the file is the one that is kept runnable**, and it is the only block
> here that a reader should copy today.

---

## 1. End-to-end, process against process

`utils/e2e_baseline.py` — `subprocess` + `os.wait4`, best of 3 after a
warm-up, per-child peak RSS. **This is the table the phase's targets are
measured against**, because "end-to-end" in the
[milestone baseline](../README.md#what-shipped) means a
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
> [S1a.6.2](../README.md#s1a62--memory-layout) needs attributed. `dwarf,8192`
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
[S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts)** — 17 430 → **305**, against
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

**The number [P1a.7](../README.md#p1a7--parallelism) asked for: a fork
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

> **Claimed at [S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts) T1a.6.8.1**
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

> **Claimed at [S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts) T1a.6.8.2**
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
storage, and beta-memories ([S1a.6.3](../README.md#s1a63--beta-memories-f11-d1), F11 D1)
are the structural answer.

### 4. Allocator traffic — 21 % of `zebra2 -e` self time, 2.5 M allocations

`malloc` 3.0 % + `cfree` 2.9 % + `[libc.so.6]` 15.2 %, at ~53 bytes per
allocation. LBR names the callers: `compile::plan_key`'s `Vec<String>`,
`BindingKey`'s boxed register slice, `saturator::Entry`'s drop glue,
`Interner::intern`. Item 1 removes a large share of it for free (a memo hit
allocates nothing); the rest is
[S1a.6.2](../README.md#s1a62--memory-layout) T1a.6.2.3/4.

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
[Q-M1a.17](../open_questions.md#q-m1a17--win-bs--80--assumed-monotone-guards-dominate),
where Win B's own ≥ 80 % assumption met a measurement and lost. The ordered
container was chosen for determinism ([design/02](../design/02_determinism_and_order.md)),
so the fix is not "use a hash set" but "do not walk what cannot have
changed" — which is the same shape as item 2 and probably the same commit.

## 8. What this chooses for the rest of the phase

| stage | profile says | order |
|---|---|---|
| **[S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts)** (new) ✅ | items 1 + 2: 21.1 % and 9.5 %, both parity-preserving by construction, both small | **shipped 2026-08-18** — −30.5 % / −7.8 %, [§10](#10-after-s1a68--the-same-instruments-re-run) |
| **[S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)** (new, added after this list) | [§9](#9-the-fork-entry-re-derivation): 95.0 % of `zebra -e` is fork saturation and 94.6 % of that is re-derivation. The measurement and the decision run **second**; the shipping half is gated on [Q-M1a.18](../open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint) and runs last | **second** |
| [S1a.6.2](../README.md#s1a62--memory-layout) Memory layout | item 4 (21 % allocator) and item 3's `FactStore` indirection; T1a.6.2.5 gains a second reason (depth 35), and §9 adds two tasks — a system allocator (T1a.6.2.7) and a per-entering region (T1a.6.2.8) | third |
| [S1a.6.3](../README.md#s1a63--beta-memories-f11-d1) Beta-memories | **gate opens**: 66.9 % of `zebra -e` is the join, and a fork's delta is 3.6 KB, so F11's "a memory copied per fork can lose more than it saves" no longer holds. §9 gives it its target: the *root* memories are the invisible way to remove the re-derivation | fourth |
| [S1a.6.4](../README.md#s1a64--hypgen-and-lattice-hot-paths) Hypgen and lattice | 7.3 % / 5.3 % self — real, smaller than written. **T1a.6.4.1's premise needs its own measurement first**: the interning on the profile's hot list is the *compiler's* (42 % `Compiler::slot`, 33 % `Compiler::premise`), so "18 k interns per `complete()` call" is not what this profile shows | fifth |
| [S1a.6.5](../README.md#s1a65--frontend-and-load-path) Frontend and load | **already met**: `parse zebra2` 200 µs, `load` 1.04 ms, and the whole `saturate zebra2` process 5.0 ms against a ≤ 15 ms target. Reduce to a confirmation + the allocation report its acceptance asks for | sixth, short |
| [S1a.6.6](../README.md#s1a66--the-differential-fuzzer) Differential fuzzer | unchanged — it guards everything above | throughout |
| [S1a.6.7](../README.md#s1a67--re-measure-the-lever-matrix) Re-lever matrix | unchanged | last |

## 9. The fork-entry re-derivation

**Added 2026-08-18, same build and machine**, after the § 7 list was
written. It is not one of the five costs above — it is the *shape* three of
them share, and it is measured here because it is bigger than any of them
individually. [S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) is the stage;
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
> [S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator). It is kept because it is the
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
root-only premises and is still found, because [T1a.6.9.4](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator)
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
[S1a.6.2](../README.md#s1a62--memory-layout) gained T1a.6.2.7 (a system allocator
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
binary, which is how [T1a.6.9.3](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) sizes the
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
takes 43 ms off. Recorded because [rule 3](../README.md#p1a6--performance)
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
one [S1a.6.3](../README.md#s1a63--beta-memories-f11-d1) is for.

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
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 \
    --cum-of ein_infer::compile solve examples/zebra2.ein -e
cd ein.rs
cargo run --release --features counters -p ein-infer --example counter_cost
cargo run --release -p ein-infer --example alloc_cost
```

## 11. The resumed fork saturator, measured

**[S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator), 2026-08-19**, same machine. §9
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
[S1a.6.3](../README.md#s1a63--beta-memories-f11-d1)'s subject and not this stage's. Building
the layered snapshot would have been the wash
[Rule 3](../README.md#p1a6--performance) exists to prevent.

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
[`zebra_walkthrough.md`](../../../kernel/inference/zebra_walkthrough.md)
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
[S1a.6.10](../README.md#s1a610--the-parity-contract-relaxes-answers-not-narration) is the stage that teaches the harness
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
[`zebra_walkthrough.md`](../../../kernel/inference/zebra_walkthrough.md)
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
[T1a.6.11.2](../README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing) ports the assertion to ein.rs, so
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

**[S1a.6.10](../README.md#s1a610--the-parity-contract-relaxes-answers-not-narration) + [S1a.6.11](../README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing),
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
`utils/mutant_ein.py` runs the *shipping* binary and deletes one event from
the log it wrote:

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

[S1a.6.11](../README.md#s1a611--einrss-own-fixtures-for-what-parity-stopped-comparing): twelve ein.rs goldens, 2 188 lines —
five real solves' traces, two `slice` cones, a fork's own `enterings/` dump
with the timeline's firing counts, the snapshot projection, and three event
streams that between them contain every class the relaxed T2 elides.
`./run_tests.sh` gained a **Phase 3** (`cargo test --workspace`) so the repo's
one documented gate runs both engines: **1 506 + 21 + 302** green.

### Reproducing this section

**From the repo root, and that is not a style choice:** the harness runs both
implementations with the repo root as their working directory
([conformance/README](../../../../corpus/README.md)), so a *relative* impl
path is resolved against the root no matter where the runner was launched.
`cd ein.rs` plus `--impl-b ./target/release/ein` is the shape that looks right
and reports 473 harness errors — which the liveness check catches, loudly, and
which is why it exists.

```sh
cargo build --release --manifest-path ein.rs/Cargo.toml

# the two tiers, relaxed (the shipping contract)
for T in T3 T2; do
  ein.rs/target/release/ein-conformance run --tier $T \
      --impl-a ".venv-pypy/bin/python -m ein.cli" \
      --impl-b ein.rs/target/release/ein
done

# the determinism sweep, unrelaxed
ein.rs/target/release/ein-conformance run --tier T3 --strict \
    --impl-a ".venv-pypy/bin/python -m ein.cli" \
    --impl-b ".venv-pypy/bin/python -m ein.cli" \
    --env-a PYTHONHASHSEED=0 --env-b PYTHONHASHSEED=42

# the negative control
for M in productive redundant enqueue; do
  EIN_MUTANT=$M ein.rs/target/release/ein-conformance run --tier T2 \
      --filter branching \
      --impl-a "$PWD/.venv-pypy/bin/python -m ein.cli" \
      --impl-b "python3 $PWD/utils/mutant_ein.py $PWD/ein.rs/target/release/ein"
done
```

## 13. S1a.6.2 — the layout stage, and the profile it starts from

**2026-08-19, `master` @ `66f24d5`, same machine.** [Rule
6](../README.md#p1a6--performance) says a stage begins by re-running
[S1a.6.1](../README.md#s1a61--fresh-profile-and-bench-baseline)'s instruments rather than by trusting
the last stage's table, and this time it rewrote the stage. Four of
[S1a.6.2](../README.md#s1a62--memory-layout)'s eight tasks were written against a
profile in which the allocator was 21 % of self time over 2.5 M allocations
averaging ~53 bytes; [S1a.6.8](../README.md#s1a68--the-compile-cache-and-the-extent-counts) and
[S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) have since removed **half of every
allocation the engine makes**, and the ones they removed were the small ones.

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
peak RSS** on `zebra -e`, which the stage's acceptance names as the one thing
that may not get worse; `jemalloc` keeps the RSS, returns a third less of the
win, and gates itself on `cfg(not(target_env = "msvc"))` by its own README,
where [P1a.9](../README.md#p1a9--release) ships a Windows binary.

**snmalloc is not free of that charge either**, and the number is here rather
than in a footnote: it costs **+1.2 MB (+6.9 %)** on `zebra -e` and nothing on
the other five cells. Against `mimalloc`'s +42 % on the same cell that is the
trade that was taken, not a clean sheet.

¶ **The one regression, and it is start-up.** `render rules` is 1.1 ms of
which almost all is process start-up, and snmalloc's arena set-up costs
**0.5 ms** of it — measured at 21 samples, spread 4.5 %, so it is real and not
noise. Every workload with work in it repays that in the first millisecond;
`saturate zebra2`, at 4.9 ms the shortest one that saturates anything, is
already ahead.

### The same change, in process

`cargo bench`, both arms out of one tree — the bench target declares the
binary's allocator, and `-p ein-corpus --no-default-features` is the
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
is slower than glibc's fastbins. 31.6 ns × 104 forks is 3.3 µs on a run that
got 15.5 ms faster. Recorded because
[rule 3](../README.md#p1a6--performance) only works if a regression inside a
win is still written down — and half-retracted at the end of the stage, where
the same bench read **274 ns** with nothing in that path changed by
T1a.6.2.6. A 30 ns bench at this scale is measuring code alignment as much as
it is measuring an allocator.

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
([`ein.rs/Cargo.toml`](../../../../ein.rs/Cargo.toml)), and the profiling binary
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

### Reproducing the bake-off

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
    -p ein-corpus --no-default-features
python3 utils/criterion_table.py --max-rsd 3

# the attribution, and the release-vs-profiling line that has to stay ±1 %
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 solve examples/zebra.ein -e
```


### T1a.6.2.2 and T1a.6.2.6 — the candidate loop, and the two tasks that swapped places

The stage's two matcher-facing layout tasks were written as *bucket-major
storage* (make the participation index's buckets contiguous) and *row packing*
(shrink `Row` from 12 bytes to 8). Both premises were wrong, and one
instrument said so before either was built.

**Where the candidates come from** — `scan_bucket` / `scan_extent` /
`cand_bucket` / `cand_extent`, added to the counter set:

| | `zebra2 -e` | `zebra -e` |
|---|---:|---:|
| candidates | 4 570 900 | 25 160 149 |
| …from a participation-index bucket | 107 193 (**2.3 %**) | 225 784 (**0.9 %**) |
| …from a full extent scan | 4 463 707 | 24 934 365 |
| bucket scans / extent scans | 39 994 / 73 642 | 171 189 / 67 803 |
| mean extent walked | **61** | **368** |

**The index does not key a nested-fact argument** — `index_fact`'s "the
join-key types only", which is ein.py's S1.8.B-idx reproduced — and
`(not (R …))` is what the corpus scans. So bucket-major storage would have
been built for **0.9 %** of `zebra -e`'s candidates. T1a.6.2.2 is closed
against that number, and the finding it leaves behind is
[S1a.6.3](../README.md#s1a63--beta-memories-f11-d1)'s: the extent scan is not slow, it is
*unnecessary*, and what would remove it is an index that reaches inside a
nested argument.

**What a candidate costs** — `examples/layout_shape.rs`, the distributions
T1a.6.2.3 asks for:

| | `zebra2` | `zebra` |
|---|---:|---:|
| facts interned, whole run | 558 | 1 104 |
| the fact store | **12 KB** | **22 KB** |
| arity ≤ 2 | 83.5 % | **96.6 %** |
| registers per plan (max) | 5 | 5 |
| premises per disjunct (max) | 3 | 1 |
| facts per fork KB (mean) | 418 | 581 |
| layers per fork KB (mean / max) | 23.8 / 35 | 24.1 / **34** |

**The store has been in L1 the whole time**, and it does not grow between a
fast solve and an exhaustive one. No cost in this engine is a fact-store
cache-footprint cost, which retires T1a.6.2.2's SIMD-precondition argument and
T1a.6.2.6's cache-line arithmetic in one line. What is left is the
**dependency chain**: `rows[id]` then `args[row.args_at]`, a load whose
address the previous load produces, twice per candidate — once for the
premise and once for the fact inside its nested pattern.

So the row got *bigger*, not smaller: **20 bytes, holding up to two arguments
inline**. Three parts, measured in the order they were built, because the
middle one is why they are one change:

| build | `zebra2 -e` | `zebra -e` | note |
|---|---:|---:|---|
| after T1a.6.2.7 | 82.7 ms | 366.0 ms | |
| + `unify` as a slice zip | 82.2 ms | 364.4 ms | −0.68 % `solve_fast`, p = 0.00 |
| + the inline row | 82.8 ms ❌ | 353.0 ms | **+1.6 % on `zebra2`** (criterion, p = 0.00) |
| + relation first, arguments second | 76.1 ms | 366.2 ms ❌ | `zebra`'s win gone: two row loads |
| **+ one row read, then the branch** | **75.7 ms** | **348.8 ms** | both |

The middle two rows are the point. `zebra2` dies on the nested relation
comparison — **79 %** of its candidates — and an inline row makes resolving
an argument list they never read *more* expensive, not less; `zebra`'s 25 M
candidates almost all pass that comparison and want the arguments
immediately. Asking `FactStore::rel` and then `FactStore::args` serves the
first and loses the second, because it loads the row twice. `FactStore::row`
+ `args_of` — one load, then the branch — serves both, and exists for exactly
that caller.

`INLINE_ARGS = 2` is the histogram's number, and `inline_share()` is how it
stays one rather than becoming a guess again.

### T1a.6.2.5 — the flatten threshold was never built, and building it costs 7.6 %

[design/03 §5](../design/03_data_model.md) specifies "flatten when a delta
grows past a threshold"; P1a.2 shipped the layered KB without one, and
`Kb::flatten` has exactly one caller, a test. So there was no threshold to
sweep — the question is whether flattening pays at all, and a search at
**depth 24 (mean), 34 (max)** whose every `facts_of` chains that many layers
looks like it should.

The experiment: a KB-level `flat_by_rel` maintained on insert and cloned per
fork, so a relation's whole extent is one flat vector and `facts_of` is one
hash lookup. Identical output, **identical work counters**, +3 682
allocations (the per-fork clone).

| | before | flat | change |
|---|---:|---:|---:|
| `match_hot/zebra2` | 38.1 µs | **35.0 µs** | **−8 %** |
| `boundary/zebra` | 6.69 ms | **6.24 ms** | −6.7 % |
| `boundary/zebra2` | 2.10 ms | 2.00 ms | −4.8 % |
| `fork/zebra2` | 302 ns | 424 ns | +40 % |
| **`solve zebra -e`** | 350.3 ms | **376.8 ms** | **+7.6 %** |
| `solve zebra` | 89.1 ms | 96.9 ms | +8.8 % |
| `solve zebra2 -e` | 77.1 ms | 77.8 ms | +0.9 % |

**Every bench that does not fork got faster and the search got slower**, which
is the whole result. The `fork` regression is real but too small to be the
cause (122 ns × 114 forks = 14 µs against 27 ms), and a control build that
*maintains and clones* the flat map while still reading through the layered
path costs **0.3 %** — so the 7.6 % is in the reading, not the copying.

The mechanism the control isolates: **a fork shares its parent's index
memory.** `by_rel`'s vectors live in sealed layers behind an `Arc`, so the
~450-fact extent the matcher scans is *one* copy that every one of the 24 live
KBs on the search stack reads. A flat index gives each fork its own copy of
that extent to fill a cache with. `match_hot` never forks, which is why it
measured the opposite sign.

Reverted, per [rule 3](../README.md#p1a6--performance). design/03 §5's
threshold is answered with a number rather than left open: **do not flatten**,
and the reason generalises to [P1a.7](../README.md#p1a7--parallelism), where
sharing the base index across workers is worth more than shortening a chain.

### T1a.6.2.1, T1a.6.2.3, T1a.6.2.4 and T1a.6.2.8 — closed against numbers

| task | closed by |
|---|---|
| **T1a.6.2.1** the participation-index key | the index carries **0.9 %** of `zebra -e`'s candidates and 2.3 % of `zebra2 -e`'s. Hashing a 12-byte key better, or splitting it per relation, cannot reach a run through 1 % of it. The *contents* question — that a nested argument is not keyed at all — is real and is [S1a.6.3](../README.md#s1a63--beta-memories-f11-d1)'s |
| **T1a.6.2.3** `SmallVec` sizing | the distributions are printed above; nothing on the hot path allocates. `tests/match_alloc.rs` already holds the matcher to zero allocations per candidate, and the allocation callers in the row below say the traffic that remains is *copied live state*, which an inline capacity cannot remove |
| **T1a.6.2.4** arena reuse / **T1a.6.2.8** the per-entering region | after T1a.6.2.7 the allocator is **3.2 %** of `zebra -e` and **7.8 %** of `zebra2 -e`, and `--callers` puts it in the per-entering snapshot rather than in scratch: `Entry` drop glue is 44 % of the deallocations, `Vec::clone<Entry>` (from `Saturator::resume`) 1.0–1.6 % self, plus the `GuardSetId` and `BindingKey` table clones. A region would absorb those — it is the right shape — but its whole ceiling is now those few per cent, against threading an arena lifetime through the engine. **Parked with the number**, to be re-priced after S1a.6.3 moves the mix again |

### The bench set, end of stage

`cargo bench` after the two changes, against the S1a.6.8 column
[§10](#10-after-s1a68--the-same-instruments-re-run) recorded — the gap
includes S1a.6.9, which never took a criterion column of its own:

| bench | at S1a.6.8 | at T1a.6.2.7 | **end of S1a.6.2** | vs S1a.6.8 |
|---|---:|---:|---:|---:|
| `parse/corpus` | 765.0 µs | 749.9 µs | **739.2 µs** | −3.4 % |
| `parse/zebra2` | 197.5 µs | 189.8 µs | **187.7 µs** | −5.0 % |
| `parse/zebra2_resolve` | — | 755.1 µs | **758.0 µs** | — |
| `load/zebra2` | 1.014 ms | 910.8 µs | **898.4 µs** | −11.4 % |
| `saturate_root/zebra2` | 2.700 ms | 2.10 ms | **1.99 ms** | −26.3 % |
| `match_hot/zebra2` | 39.3 µs | 38.1 µs | **35.3 µs** ‖ | −10.2 % |
| `boundary/zebra` | 7.252 ms | 6.69 ms | **6.37 ms** | −12.1 % |
| `boundary/zebra2` | 2.710 ms | 2.10 ms | **2.00 ms** | −26.2 % |
| `fork/zebra2` | 268.9 ns | 302 ns | **274 ns** | +1.9 % |
| `solve_fast/zebra2` | 28.37 ms | 20.12 ms | **18.21 ms** | −35.8 % |
| `solve_exhaustive/zebra2` | 133.10 ms | 79.89 ms | **73.51 ms** | −44.8 % |

‖ `match_hot` read 3.98 % relative sd in the full-set run and **0.30 %** when
re-run alone on a quiet machine (35.04 / 35.15 / 35.25 µs over three runs). At
35 µs it is the shortest bench in the set and the most sensitive to what the
machine was doing a second earlier; the gate reading is the quiet one, and the
noisy one is recorded next to it rather than dropped.

### Where `zebra -e` stands

| subsystem | at S1a.6.9 | at T1a.6.2.7 | **after T1a.6.2.6** |
|---|---:|---:|---:|
| match/bind | 80.1 % | 86.5 % | **84.8 %** |
| saturate | 13.6 % | 10.0 % | 11.1 % |
| allocator | 9.4 % | 3.0 % | 3.2 % |
| hypgen/branch | 4.3 % | 1.7 % | 2.2 % |

`unify` 49.3 %, `try_candidate` 16.7 %, `walk` 9.4 % — **75.4 % of the run in
three functions of the join**, and the counters say why: 25.16 M candidates,
99.1 % of them from a 368-fact extent scan, 2 slot unifications each. Nothing
about that is a layout problem any more. It is
[S1a.6.3](../README.md#s1a63--beta-memories-f11-d1)'s.

### Reproducing this section

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example layout_shape                       # the distributions
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example counter_cost          # scan_bucket / cand_extent / nested_rel_*
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 8 --callers alloc \
    solve examples/zebra.ein -e                  # who allocates
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml -p ein-corpus \
    --bench engine -- solve --save-baseline before   # then edit, then --baseline before
```

## 14. S1a.6.3 — the alpha-memory, and the gate the beta-memory did not pass

**2026-08-19, `master` @ `14a7c47`, same machine.** The stage that was going to
build beta-memories built two changes to the index instead, and then answered
its own gate with the profile they left behind. `solve zebra.ein -e`
**349.1 → 78.1 ms (4.5×)** and `solve zebra2.ein -e` **75.8 → 44.0 ms (1.7×)**.

| workload | target | at S1a.6.2 | **at S1a.6.3** | vs PyPy today |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | ≤ 200 ms | 75.8 ms | **44.0 ms** ✅ | **112×** |
| `solve zebra.ein -e` | ≤ 400 ms | 349.1 ms | **78.1 ms** ✅ | **112×** |
| `solve zebra2.ein` | — | 20.5 ms | **10.9 ms** | 232× |
| `solve zebra.ein` | — | 89.2 ms | **14.8 ms** | 206× |
| parse + load `zebra2` | ≤ 15 ms | 0.90 ms | **0.90 ms** ✅ | 478× |
| the acceptance gate (3 fixtures) | ≤ 5 s | 0.58 s | **0.28 s** ✅ | **129×** |

### T1a.6.3.0 — the index keys one level in

[§13](#13-s1a62--the-layout-stage-and-the-profile-it-starts-from) measured that
**99.1 %** of an exhaustive `zebra`'s 25.16 M candidates came from a full
extent scan, because `index_fact` keys "the join-key types only" — a
`Fact`-valued argument is not keyed — and the premise that does the scanning is
`stdlib/slots.ein`'s

```
:match (and (?R ?a ?b) (not (?R ?b ?i)) (?isa ?i ?index))
```

whose middle premise walks `not`'s 368-fact extent to reject all but a handful.
The index now holds `(rel, slot, inner) = value` one level inside a nested
argument, and `probes_for` compiles a probe per inner slot.

| counter | `zebra2 -e` before | after | `zebra -e` before | after |
|---|---:|---:|---:|---:|
| `candidates` | 4 570 900 | **306 725** | 25 160 149 | **1 171 385** |
| …from an extent scan | 4 463 707 | **103** | 24 934 365 | **581** |
| `unify_slot` | 5 933 579 | **929 997** | 51 452 037 | **3 474 509** |
| `unify` | 5 617 846 | 614 264 | 50 213 778 | 2 236 250 |
| `walk` | 355 843 | **355 843** | 530 405 | **530 405** |
| `plan_run` | 123 254 | **123 254** | 104 409 | **104 409** |
| `binding_key` | 12 694 | **12 694** | 31 563 | **31 563** |
| `fact_insert` | 2 184 | **2 184** | 6 766 | **6 766** |
| `guard_query` | 30 691 | **30 691** | 29 865 | **29 865** |
| `watch_stamp` | 204 158 | **204 158** | 248 043 | **248 043** |
| `fork` / enterings | 104 / 101 | **104 / 101** | 114 / 111 | **114 / 111** |

**Every counter that measures a decision the engine made is identical to the
digit; every counter that measures candidates it was offered fell by 93–95 %.**
That split is the whole claim of a narrowing, and it is why T2 stayed at
**239/240** and T3 at **472/473** with [D2](../divergences.md) the only cell in
either.

**Why it is sound** (T1a.6.3.1's argument, written before the code): a probe
replaces "every fact of the relation" with "every fact in a bucket", and the
emitted sequence is unchanged iff the bucket **contains every fact that would
have unified** — it does, because the key is exactly one of the equalities
`unify` will re-check, and the matcher re-checks *all* of them regardless — and
**yields them in extent order** — it does, because buckets are appended in
insertion order and read oldest-layer-first, exactly like the extent. Only the
count of *rejected* candidates differs, and that is ein.rs's own instrument
rather than an observable.

**Why it is checked, not argued**:
`match_::tests::narrowing_never_changes_the_match_sequence` runs both index
write paths — the batch `rebuild_layer` and the incremental `index_fact`, which
is the one a fork writes through — over **16 randomised insertion orders**,
with the narrowing on and off, and compares the match sequences. It was
verified to fail when either path keys the wrong inner position, which is the
only way to know a differential test is testing anything.

**One observable did move, and it was not a match.** `saturate`'s snapshot
prints `facts_by_rel_slot_val`, and ein.rs's index now holds more postings than
ein.py's — 897 against 743 on `zebra2-hints` — which broke **43** T3 cells.
That line reports a property of the *knowledge base*: how many join keys its
facts produce. How an engine chooses to index those facts to answer a query
faster is not what a reader of the snapshot is being told, so `index_sizes`
counts the `DIRECT` postings only and says why in the code.

### T1a.6.3.0b — and then the lookup became the cost

With the bucket lookup now the common case, `Kb::facts_with` was **15.6 %** of
`zebra -e`: a fork 24 layers deep hashes its key 24 times to find the one or
two layers that hold it. Each layer carries a 2048-bit Bloom filter over its
participation-index keys, so the walk hashes once and bit-tests per layer.

| bits per layer | `solve zebra -e` |
|---:|---:|
| none | 83.4 ms |
| 512 | 78.4 ms (−6.0 %) |
| **2048** | **77.6 ms (−7.3 %)** |
| 8192 | 77.7 ms (−7.2 %) |

Flat past 2048, so 256 bytes a layer — 4 % of the ~6 KB a fork owns, and
`Layer::footprint` counts it so that number stays honest. `facts_with` is
**10.1 %** after. A filter can only skip a layer that *provably* lacks the key,
and the failure mode that would matter — a stale filter silently dropping
matches — is guarded by a `debug_assert` in `facts_with`, by the differential
test, and by T2.

### The bench set

| bench | at S1a.6.2 | **at S1a.6.3** | change |
|---|---:|---:|---:|
| `parse/corpus` | 739.2 µs | 748.1 µs | +1.2 % |
| `parse/zebra2` | 187.7 µs | 190.8 µs | +1.7 % |
| `load/zebra2` | 898.4 µs | 904.5 µs | +0.7 % |
| `saturate_root/zebra2` | 1.99 ms | **1.54 ms** | −22.4 % |
| `match_hot/zebra2` | 35.3 µs | **24.4 µs** | **−30.9 %** |
| `boundary/zebra` | 6.37 ms | **1.65 ms** | **−74.1 %** |
| `boundary/zebra2` | 2.00 ms | **1.54 ms** | −22.8 % |
| `fork/zebra2` | 274 ns | 290 ns | +5.8 % |
| `solve_fast/zebra2` | 18.21 ms | **8.41 ms** | **−53.8 %** |
| `solve_exhaustive/zebra2` | 73.51 ms | **40.17 ms** | **−45.4 %** |

11 benches, worst relative sd **1.25 %**. `boundary/zebra` at 3.9× is the row
that names the win: the NAF boundary re-runs negative queries, and a negative
query is exactly a `(not …)` scan. The three frontend rows moved by +1 % on a
path this stage does not touch — that is the machine, and it is what a spread
column is for. `fork` pays 16 ns for a bigger index to copy.

### Where `zebra -e` stands now

| subsystem | at S1a.6.2 | **at S1a.6.3** |
|---|---:|---:|
| match/bind | 84.8 % | **37.7 %** |
| saturate | 11.1 % | **47.4 %** |
| hypgen/branch | 2.2 % | 7.7 % |
| allocator | 3.2 % | ~12 % |

| symbol | share |
|---|---:|
| `Matcher::unify` | 11.6 % |
| `Kb::facts_with` (the layer walk, both closures) | 10.1 % |
| `Saturator::admit_from_boundary` | 7.3 % |
| `Matcher::walk` | 6.8 % |
| `Vec::clone ⟨Entry⟩` (the per-entering snapshot) | 3.7 % |
| `Matcher::try_candidate` | 3.7 % |
| `btree::set::Iter::next` (the parked set) | 3.1 % |

The join is no longer the phase's subject. The run is 4.5× shorter and its two
biggest remaining blocks are the **boundary** (`admit_from_boundary` + the
ordered walk of `parked` ≈ 10.4 %) and the **allocator** (~12 %, of which the
per-entering snapshot's copies are the largest single caller).

### Memory

| cell | allocations | churn | peak live | per-fork delta (mean / max) |
|---|---:|---:|---:|---:|
| `zebra2 -e` | 881 523 | 63.0 MB | 2.70 MB | **4.5 K** / 11.7 K |
| `zebra -e` | 1 678 816 | 128.2 MB | 3.31 MB | **6.2 K** / 10.8 K |

Allocations +0.26 % against S1a.6.2 — the new index entries — and the per-fork
delta 3.9 → **6.2 KB** on `zebra -e`, which is the number
[P1a.7](../README.md#p1a7--parallelism) sizes `--jobs` by: a thousand
concurrent searches is 6 MB of deltas rather than 4. Process peak RSS is
unchanged at 17.5 MB.

### The gate: the beta-memory is **not** built, and here is the number

The stage ships a beta-memory only if it is T2-green **and** measurably better
on both puzzles, and it says in as many words that failing that test and
recording why "is a successful outcome for this stage". It fails it, and the
reason is what the two changes above did to the thing a beta-memory
materialises.

**A beta-memory exists to stop re-enumerating a large intermediate.** The
intermediate is now 2.2 tuples wide:

| | before T1a.6.3.0 | after |
|---|---:|---:|
| candidates per step entered (`zebra -e`) | 25 160 149 / 530 405 = **47.4** | 1 171 385 / 530 405 = **2.21** |
| candidates per step entered (`zebra2 -e`) | 4 570 900 / 355 843 = 12.8 | 306 725 / 355 843 = **0.86** |

A table lookup replacing 47 candidates is a lever; a table lookup replacing 2.2
is a constant factor with a per-fork table attached. And the cost side is not a
guess either: [T1a.6.2.5](#t1a625--the-flatten-threshold-was-never-built-and-building-it-costs-76)
built the exact shape design/05 §7 calls the *root memory* — a flat per-relation
table — and cloning it per fork cost **7.6 %** while making the fork-free bench
8 % faster. F11's original objection ("a memory that must be copied per fork can
lose more than it saves") now has a measurement under it rather than a
suspicion.

**F11 D1 is therefore updated, not landed**, and the three tasks that were the
memory itself are closed against numbers:

| task | outcome |
|---|---|
| T1a.6.3.1 the ordering argument | **written and executed** — and it is the argument the *narrowing* needed, so it was worth writing whatever the memory's fate |
| T1a.6.3.2 root memories | **moot**: [S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) made a fork resume root's saturation instead of replaying it, so there is no root re-derivation left to replay from a table |
| T1a.6.3.3 fork delta memories | **not built**: the join inside a fork's delta is 2.2 candidates per step |
| T1a.6.3.4 which prefixes to materialise | **not reached** |
| T1a.6.3.5 guards get no memories | **stands** — and the boundary is now the biggest single block, so if this returns it returns there |
| T1a.6.3.6 feature-gate and measure | **not needed**: nothing was gated, and the A/B was two builds |

**Q-M1a.10** asked whether the beta-memory is still the largest lever after the
register matcher and the semi-naive boundary. Answered: **no.** It was 66.9 %
of `zebra -e` when the phase was planned; the same subsystem is 37.7 % of a run
that is 7.5× shorter, and what took it there was an index key and a Bloom
filter, neither of which is a memory.

### D2's trigger, re-checked — and design/05 was wrong about the shape

The stage asks to re-check the worst-case-optimal join's trigger and record the
answer without implementing it. [design/05
§6](../design/05_matcher.md) rejects D2 partly on "ein's rule bodies are
acyclic chains/stars where a left-deep binary plan is already optimal". **The
first half of that is false.** `stdlib/slots.ein`'s `slot-adjacent-fwd` binds

```
(?S ?a ?b) (?R ?b ?p1) (?isa ?p1 ?PT) (?S ?p2 ?p1) (?isa ?p2 ?PT)
```

whose variable graph contains the triangle `p1 — PT — p2 — p1`: a cyclic
body, and the classic shape a Generic Join beats a binary plan on.

The *cost* half of the trigger is still unmet, which is why D2 stays out of
scope: the relations that triangle ranges over are `instance` (30 facts) and
`right-of` / `next-to` (16) on `zebra`, so the AGM bound and the binary plan are
within a small constant of each other, and matching is 37.7 % of a run that is
now 78 ms. Recorded so the *next* re-check asks about the cost rather than
re-deriving the shape — and so design/05 stops claiming a property the corpus
does not have.

### Reproducing this section

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example counter_cost     # the candidate collapse
cargo test --manifest-path ein.rs/Cargo.toml -p ein-infer --lib narrowing
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 9
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 20 solve examples/zebra.ein -e
python3 utils/fork_split.py                # the redundant firings, unmoved
```

## 15. S1a.6.4 — the per-call setup, and the enumerator the targets never run

**2026-08-19, `master` @ `d40b1c0`, same machine.** The stage that was written
against "18 k raw candidates per call" found **125**, and found the cost
somewhere else entirely: in what a hypothesis-generation *call* sets up before
it looks at a candidate, and in the blind enumerator that **no milestone
workload reaches**. `solve examples/features/05_stdlib_domain_elim.ein -e`
**4182.1 → 3559.7 ms (−14.9 %)**, `features/01 -e` **−15.1 %**,
`branching/07 -e` −9.5 %; the four targets move by 1–2 % (and `solve zebra2`,
which is not one, by 5.5 %) and stay met with room.

### The four targets

Both columns are best-of-15 processes measured today, so the middle one is a
re-measurement of `322dd63` rather than S1a.6.3's recorded value — the machine
was busier when this stage started than when the last one ended, and Rule 6
exists because that is normal.

| workload | target | at S1a.6.3 (re-run) | **at S1a.6.4** | vs PyPy ¤ |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | ≤ 200 ms | 42.7 ms | **41.7 ms** ✅ | **118×** |
| `solve zebra.ein -e` | ≤ 400 ms | 78.1 ms | **77.1 ms** ✅ | **114×** |
| `solve zebra2.ein` | — | 10.9 ms | **10.3 ms** | 246× |
| `solve zebra.ein` | — | 15.1 ms | **15.1 ms** | 202× |
| parse + load `zebra2` | ≤ 15 ms | 0.90 ms | **0.89 ms** ✅ | 483× |
| the acceptance gate (3 fixtures) | ≤ 5 s | 0.199 s | **0.196 s** ✅ | **184×** |

¤ **Against [§1](#1-end-to-end-process-against-process)'s PyPy column**
(4 938.0 / 2 529.9 / 8 787.4 / 3 045.1 ms), which is what §10–§14 divide by.
The [README's target table](../README.md#p1a6--performance) carries a *different* PyPy
reading from the same stage — 4.53 s and 8.33 s — so its ratio column reads
~5 % lower for the same ein.rs number (109× and 108× for this row). Both are
real and neither is wrong; they are two runs of the same thing, and the point
of naming the denominator is that a reader can tell which is which.

The acceptance row is a **fourth** reading of a quantity §6 already says has
three, and it is smaller than S1a.6.3's 0.28 s because it is measured
differently: the three `ein-infer` acceptance tests run from their own binary,
unpinned, best of three, at *both* ends of this stage's own A/B (0.199 →
0.196 s). The delta is what the stage claims; the absolute is what the method
gives.

### The cells the targets do not cover

Every one of the four is `(hrule …)`-driven, so `generate` returns before
`candidate_objects` and **none of them runs the blind enumerator at all**. The
corpus's slowest `solve` cells all do, and `features/05 -e` alone is 46× `solve
zebra -e`:

| cell | at `322dd63` | **at S1a.6.4** | change | peak RSS |
|---|---:|---:|---:|---:|
| `features/05_stdlib_domain_elim -e` | 4182.1 ms | **3559.7 ms** | **−14.9 %** | 445 MB |
| `features/01_not_and_absent -e` | 2184.0 ms | **1854.9 ms** | **−15.1 %** | 724 MB |
| `branching/07_lookahead_off -e` | 1024.1 ms | **927.2 ms** | −9.5 % | 56 MB |
| `branching/06_lookahead_on -e` | 217.6 ms | **208.0 ms** | −4.4 % | 62 MB |
| `saturation/square-bwd/houses -e` | 272.2 ms | **260.8 ms** | −4.2 % | 97 MB |

Best-of-5, spreads 0.2–1.2 % — these cells are seconds long and are a far
quieter measurement surface than the 42 ms one the targets are written on.
`utils/e2e_baseline.py --blind` is now the row set, so the next stage does not
have to rediscover them.

**Two of them are also where the memory is**, which nothing in this phase had
looked at outside the four cells: `features/01 -e` peaks at **724 MB**, and
uncapped `saturation/square-unique/terminus.ein -e` — a corpus file whose
`solve` runs are deliberately *not* in `corpus.toml`, "a run nobody can finish
is not coverage" — reaches **12.3 GB** and was OOM-killed on this machine at
108 s. It is ~1 KB per entering and 12 M enterings; capped, the growth is
exactly linear (400 k enterings → 404 MB, 2.3 s). Unchanged by this stage and
identical on `322dd63`, so it is a property of the search rather than a
regression — recorded because [P1a.7](../README.md#p1a7--parallelism) sizes
`--jobs` by per-search memory, and this is the number that bounds it.

> **Superseded as a bound, 2026-08-22.** The rows above stand as taken, but
> most of what they measured was one structure, and it is gone.
> [T1a.7.1.7](../README.md#s1a71--making-the-shared-state-sync)
> found that the peak was overwhelmingly a provenance arena nothing reclaimed
> until the run ended — 2 135 093 records on `features/01 -e`, twelve of them
> live — and gave it a per-worker region: the same cell now peaks at
> **85–91 MB**, `sq-bwd/houses -e` at 17 MB and `branching/07 -e` at 16 MB.
> `terminus.ein`'s 12.3 GB and the "~1 KB per entering" it implies have **not**
> been re-measured and should be before anything is sized by them
> ([shared_state.md §2c](shared_state.md#2c-what-the-region-did--the-after-column)).

### What a call costs, and what it was spending it on

`examples/hypgen_calls.rs`, the instrument this stage's acceptance asks for.
Steady state — root saturated, one warm-up pass so the kill cache is written,
which is the state every call in the search loop but the first sees:

| cell | | at `322dd63` | T1a.6.4.0 | T1a.6.4.0b | **at S1a.6.4** |
|---|---|---:|---:|---:|---:|
| zebra2 | `complete()` | 61.5 µs | 47.0 | 38.3 | **38.4 µs** |
| zebra2 | of which setup | 43.4 µs | 31.0 | 22.2 | **23.0 µs** |
| zebra2 | `open_hypotheses()` | 327 µs | 287 | 276 | **268 µs** |
| zebra | `complete()` | 29.2 µs | 23.1 | 19.0 | **18.8 µs** |
| zebra | of which setup | 12.0 µs | 9.1 | 5.8 | **6.0 µs** |
| terminus [blind] | setup | 373 ns | 312 | 236 | **234 ns** |

**The setup was 71 % of a `complete()` call on zebra2.** A call that
short-circuits on candidate #1 (S1.9.E16) still builds a fresh `Lookahead`,
and a fresh `Lookahead` walks `rules × activators` through a fresh `Engine` —
which the new `plan_key` counter prices exactly:

| counter | zebra2 -e | zebra -e | features/05 -e | branching/07 -e | zebra2 no-writeback |
|---|---:|---:|---:|---:|---:|
| `hypgen_call` | 36 | 42 | 384 173 | 10 937 | 3 483 |
| `hypgen_complete` | 34 | 40 | 384 167 | 10 931 | 3 477 |
| `lookahead_probe` | 91 | 94 | 384 608 | 0 | 3 750 |
| `plan_key` | **7 884** | 1 437 | 1 536 696 | **4** | 438 759 |
| (enterings) | 101 | 111 | 384 167 | 11 501 | 3 831 |

**219 compile-cache keys per call on zebra2, against 125 raw candidates for a
whole pass.** `branching/07` is the control: its lookahead is off, so
`Lookahead::new` never runs and its `plan_key` count is 4 for the entire
solve — which is why T1a.6.4.0/0b did nothing there and T1a.6.4.3 did 9.5 %.

Every other counter is **identical to the digit** on all four milestone cells,
before and after: the stage did the same work more cheaply, four times.

### The bench set

| bench | at S1a.6.3 | **at S1a.6.4** | change |
|---|---:|---:|---:|
| `parse/corpus` | 748.1 µs | 733.2 µs | −2.0 % |
| `parse/zebra2` | 190.8 µs | 187.1 µs | −1.9 % |
| `parse/zebra2_resolve` | — | 745.0 µs | — |
| `load/zebra2` | 904.5 µs | 891.4 µs | −1.4 % |
| `saturate_root/zebra2` | 1.54 ms | **1.29 ms** | **−16.2 %** |
| `match_hot/zebra2` | 24.4 µs | 24.5 µs | +0.4 % |
| `boundary/zebra` | 1.65 ms | 1.61 ms | −2.4 % |
| `boundary/zebra2` | 1.54 ms | **1.28 ms** | **−16.9 %** |
| `fork/zebra2` | 290 ns | 281 ns | −3.1 % |
| `solve_fast/zebra2` | 8.41 ms | **8.01 ms** | −4.8 % |
| `solve_exhaustive/zebra2` | 40.17 ms | **39.05 ms** | −2.8 % |

11 benches, worst relative sd **1.16 %** (gate 3 %). The two −16 % rows are
root saturation, and they are T1a.6.4.0b rather than anything aimed at them:
`Engine::compile_all` runs per enqueue pass, and it was cloning every `Rule`
and allocating a `Vec` per rule before compiling anything. It scales with the
rule count, which is why `boundary/zebra2` (30 rules) moved 7× as far as
`boundary/zebra` (6). The three frontend rows moved −1.4…−2.0 % on paths this
stage does not touch: that is the machine, and it is what a spread column is
for.

### Memory

| cell | allocations | churn | peak live | per-fork delta (mean / max) |
|---|---:|---:|---:|---:|
| `zebra2 -e` | 837 431 (−5.0 %) | 61.6 MB | 2.70 MB | 4.5 K / 11.7 K |
| `zebra -e` | 1 666 101 (−0.8 %) | 127.7 MB | 3.31 MB | 6.2 K / 10.8 K |

Peak live, the per-fork delta and the fact counts are unchanged to the digit —
the stage removed allocations, not state. Process peak RSS 17.3 MB.

### The four planned tasks, closed against numbers

| task | outcome |
|---|---|
| [T1a.6.4.1](../README.md#s1a64--hypgen-and-lattice-hot-paths) intern-on-probe | **not built — the premise is not true here.** ein.rs's row key *is* the identity, and `FactStore::intern` is a probe plus a push on a miss, so "probe first, materialise on survival" is one hash lookup either way. It is **0.69 %** of the blind-mode `features/05 -e` and 0.39 % of `zebra2 -e`, over 125–336 raw candidates per pass rather than the ~18 k design/07 §2 estimated. The `seen_in_call` table the task asks for is already `FxHashSet<FactId>` over the interned id, which is the open-addressed row-key table by another name |
| T1a.6.4.4 no-good bitmask | **not built, measured in its own regime.** design/07 §4 names `enable_singleton_writeback=false` on zebra2 as where clause checking dominates. It does explode as predicted — **3 831 enterings, 354 clauses, 2.38 s** — and in that run the whole apriori/no-good machinery is **0.3 %**: `filter_candidate` 0.3 %, `nogood` and `is_subset` 0.0 %, the `contradiction` bucket 0.1 % self. `admit_from_boundary` is 60.2 % of it. A `u64` mask would replace an instruction that is not being executed |
| T1a.6.4.5 incremental alive | **not built, and the profile says why.** The premise is that `_compute_alive` re-runs the generator per layer — true, and `hypgen_call − hypgen_complete` is **6** on every workload measured, against 384 167 `complete()` calls on `features/05 -e`. The `alive/closed` bucket is 0.5–2.6 % self and most of that is `solve.rs`'s own loop. The task's own gate ("only if the profile still shows `_compute_alive`") answers itself |
| T1a.6.4.6 `complete()` fast path | **recorded as wrong, and now also as unnecessary.** Reordering the enumeration to find a survivor sooner would change which kill-cache writes the lookahead makes along the way — root-visible facts. The stage made the per-candidate path cheaper instead, and the *setup* it removed was 71 % of the call |

### Where `zebra -e` stands, and what it chooses next

| subsystem | at S1a.6.3 | **at S1a.6.4** |
|---|---:|---:|
| match/bind | 37.7 % | 39.1 % |
| saturate | 47.4 % | 45.6 % |
| hypgen/branch | 7.7 % | 7.7 % |
| allocator | ~12 % | ~11 % |

| cone | `zebra -e` | `features/05 -e` (blind) |
|---|---:|---:|
| `Saturator::admit_from_boundary` | **37.6 %** | **28.5 %** |
| `Saturator::resume` (the per-entering snapshot) | **10.4 %** | **11.9 %** |
| `hypgen::generate` | 2.1 % | 17.8 % (was 24.7 %) |
| `candidate_objects` | — (hrule) | 3.1 % (was 10.7 %) |

The hypgen cone on the milestone puzzles is **2.1 %** and there is nothing
left in it worth a stage. What the profile names on *both* shapes of workload
is the same pair it named at the end of S1a.6.3: the **NAF boundary**
(`admit_from_boundary` plus the ordered walk of `parked`, 6.7 % self and 37.6 %
cumulative on `zebra -e`) and the **per-entering snapshot**
(`Vec::clone⟨Entry⟩` 3.8 % self, `Saturator::resume` 10.4 % cumulative). Both
are saturation, and neither has a stage.

### Reproducing this section

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example hypgen_calls                     # the per-call table
utils/bench_env.sh python3 utils/e2e_baseline.py --blind --runs 5
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example counter_cost        # add paths for other files:
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example counter_cost -- examples/features/05_stdlib_domain_elim.ein
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 2 \
    --cum-of hypgen --cum-of candidate_objects --cum-of admit_from_boundary \
    solve examples/features/05_stdlib_domain_elim.ein -e
# the no-good regime: zebra2 with :enable-singleton-writeback false
sed 's/:enable-pre-branch-lookahead true)/:enable-pre-branch-lookahead true\n  :enable-singleton-writeback false)/' \
    examples/zebra2.ein > /tmp/zebra2-nowb.ein
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 3 \
    --cum-of apriori --cum-of nogood --cum-of admit_from_boundary \
    solve /tmp/zebra2-nowb.ein -e
```

## 16. S1a.6.5 — the load path, and the modules it parsed twice

**2026-08-19, `master` @ `358e1c5`, same machine.** The stage
[§8](#8-what-this-chooses-for-the-rest-of-the-phase) shortened to "a
confirmation plus the allocation report", because its acceptance was already
met by 8×. The confirmation found that a load parsed **3.30× the bytes on
disk**: import resolution parses a module once per *edge*, and the corpus's
import trees are diamonds.

### The four targets

Frontend-only changes, so the targets are where they were and the row that
moves is the one that is nearly all frontend. The six *process* rows are
best-of-15 **in one series with both binaries present** — the S1a.6.4 one built
from `99fac86` in a worktree and passed as `--bin` — so they are an A/B rather
than two readings taken on different days. The `parse + load` row is
`cargo bench`'s `load/zebra2` and the acceptance row is the three-fixture test
binary, each measured the way [§15](#15-s1a64--the-per-call-setup-and-the-enumerator-the-targets-never-run)
measured it.

| workload | target | at S1a.6.4 | **at S1a.6.5** | vs PyPy ¤ |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | ≤ 200 ms | 41.4 ms | **40.8 ms** ✅ | **121×** |
| `solve zebra.ein -e` | ≤ 400 ms | 77.0 ms | **76.9 ms** ✅ | **114×** |
| `solve zebra2.ein` | — | 10.3 ms | **10.0 ms** | 253× |
| `solve zebra.ein` | — | 14.7 ms | **14.7 ms** | 207× |
| parse + load `zebra2` | ≤ 15 ms | 0.89 ms | **0.66 ms** ✅ | 648× ⁂ |
| the acceptance gate (3 fixtures) | ≤ 5 s | 0.196 s | **0.196 s** ✅ | 184× |
| `saturate zebra2` | — | 3.9 ms | **3.6 ms** | — |
| `render rules zebra2` | — | 1.6 ms | **1.5 ms** | — |

¤ Against [§1](#1-end-to-end-process-against-process)'s PyPy column, as
§10–§15 divide by; the [README's table](../README.md#p1a6--performance) carries a second
reading of the same interpreter and its ratios run ~5 % lower.

⁂ Against the README's 0.43 s, which is [§1](#where-the-milestones-denominators-moved)'s
account of a planned denominator that is not reproducible from its own
components — the target is met on any reading, and a whole `saturate zebra2`
*process* is now 3.6 ms.

The stage takes **0.23 ms off every invocation**, and the table is what that
looks like from both ends: 6–8 % of a process that only loads and saturates,
0.3 % of one that searches. The acceptance row is unmoved because three
`zebra2` solves pay it three times — 0.7 ms of 196.

### The load, phase by phase

`examples/frontend_cost.rs`, the instrument this stage's acceptance asks for.
Best of 50, **System allocator** (an example that counts allocations must not
also link the allocator whose job is to make them cheap), so the absolute times
run ~20 % above `cargo bench`'s snmalloc rows and the *shares* are the point:

| phase | before | **after** | allocs | share of the load |
|---|---:|---:|---:|---:|
| `read` | 5.0 µs | 4.9 µs | 1 | 0.6 % |
| `parse` (the puzzle's own) | 196.1 | **163.5** | 421 | 20.3 % |
| `resolve imports` | 602.7 | **407.2** | 1 572 | 50.6 % |
| ↳ of which macro expansion | — | 30.8 | 790 | 3.8 % |
| `Resolver::new` ×2 | 6.2 | 6.3 | 46 | 0.8 % |
| the S1.8a.f20 macro guard | 10.8 | 9.8 | 75 | 1.2 % |
| ingest (residual) | 169.7 | **146.2** | 1 873 | 18.2 % |
| `rebuild_indexes` | 34.3 | 34.5 | 586 | 4.3 % |
| `detect_provenance_cycles` | 1.2 | 1.3 | 87 | 0.2 % |
| **load (whole)** | **1022.9 µs** | **804.4 µs** | **5 451** | 100 % |

`zebra.ein` is 720.1 µs / 4 570 allocations and `features/05` 127.1 µs / 1 264.
Churn is 474 KB, 422 KB and 86 KB. The arenas a `zebra2` parse builds are
**1 111 nodes, 619 args and 157 symbols** — 23.3 source bytes per node, which
is the number T1a.6.5.2's pre-sizing was computed from and the reason it did
not pay.

### The diamond, counted

`parse_call` / `parse_bytes`, six frontend counters added here and compiled out
by default like the other twenty-four:

| load | `parse_call` | `parse_bytes` | vs the file | `lex_match` | `lex_symbol` | `intern` (miss) |
|---|---:|---:|---:|---:|---:|---:|
| `zebra2` before | 8 | 85 412 | **3.30×** | 14 304 | 1 738 | 2 743 (288) |
| `zebra2` after | **5** | **59 757** | 2.31× | 10 110 | 1 250 | 1 900 (288) |
| `zebra` after | 5 | 69 887 | 3.68× | 10 272 | 1 254 | 1 890 (284) |
| `features/05` after | 4 | 8 928 | 2.70× | 1 523 | 178 | 256 (72) |

`zebra2` imports `std.algebra` (23 623 B) and `std.bijection` (8 183 B),
`std.bijection` imports `std.algebra`, and all three import `std.macro`
(1 016 B) — so `std.macro` was parsed four times and `std.algebra` twice.
`zebra`'s tree shares only `std.macro` (`std.algebra` and `std.slots`, 25 265 B,
side by side), so its diamond is worth 1 KB and its ratio barely moves: its
`imports` phase stays 62.6 % of a load because those really are bytes it has to
parse. **The size of this win is a property of the import graph, not of the
loader** — which is why the cache alone was −24.1 % on `zebra2`'s
parse + resolve and a wash on a `zebra` load.

The one repeat left is the macro guard's, which builds its own `Ast` (9.8 µs).
`intern_miss` is **288 before and after**, which is the check that the cache
changed the parsing and not the program.

### The bench set

| bench | at S1a.6.4 | **at S1a.6.5** | change |
|---|---:|---:|---:|
| `parse/corpus` | 733.2 µs | **623.0 µs** | **−15.0 %** |
| `parse/zebra2` | 187.1 µs | **146.4 µs** | **−21.7 %** |
| `parse/zebra2_resolve` | 745.0 µs | **509.5 µs** | **−31.6 %** |
| `load/zebra2` | 891.4 µs | **664.1 µs** | **−25.5 %** |
| `saturate_root/zebra2` | 1.29 ms | 1.29 ms | −0.1 % |
| `match_hot/zebra2` | 24.5 µs | 25.0 µs | +1.7 % |
| `boundary/zebra` | 1.61 ms | 1.60 ms | −0.8 % |
| `boundary/zebra2` | 1.28 ms | 1.28 ms | −0.1 % |
| `fork/zebra2` | 281 ns | 289 ns | +2.8 % |
| `solve_fast/zebra2` | 8.01 ms | 7.95 ms | −0.7 % |
| `solve_exhaustive/zebra2` | 39.05 ms | 38.65 ms | −1.0 % |

11 benches, worst relative sd **0.77 %** (gate 3 %). The four frontend rows
were re-measured at the *start* of this stage as well and came back within
0.2 % of S1a.6.4's recorded values, so the machine is in the state that stage
left it in. The seven engine rows this stage does not touch land between
−1.0 % and +2.8 %, in both directions — that is drift between two `cargo bench`
runs a day apart, and it is the scale against which this stage's two reverts
(−0.7 % and +1.2 %) were called washes.

### Where the parse's time goes, before and after

`perf` on the `parse/zebra2` bench, self time (criterion's own
`sweep_and_estimate` + `exp` is ~15 % of each column and is excluded from the
reading):

| symbol | before | **after** |
|---|---:|---:|
| `lex::skip_trivia` | 26.3 % | **14.4 %** |
| `lex::match_term` | 13.5 % | 12.3 % |
| `lex::advance_to` | — | 7.9 % |
| SipHash + `Ast::intern` | ~7 % | (removed) |
| `Parser::{symbol,var,value,alt_generic_list,kw_pair}` | ~7 % | ~7 % |

**65 % of `zebra2.ein`'s bytes are comment and blank line, and parsing it with
all of them stripped is only 12 % faster** (25 919 → 9 055 bytes, 196.1 →
175.9 µs, on the build before the lexer changes) — the two measurements
together are what say the cost was the per-character cursor and the call
frequency rather than the comment bytes, and either one alone would have been
misread. What is left
above 10 % is the backtracking itself: eleven alternatives per top-level form,
each asking for a terminal at the same position.

### Three changes built and reverted

Rule 3 says a wash is a revert; these are the numbers behind three of them.

| change | measured | why it lost |
|---|---:|---|
| `advance_to` vectorised — `is_ascii()`, `rposition`, `filter().count()` | **+10…+14 %** | the spans are one space and a two-character indent; three passes lose to one loop |
| AST arena pre-sizing from source length (T1a.6.5.2's own subject) | **+1.2…+3.0 %** | 1 111 nodes and 157 symbols: eleven doublings of 13 KB cost less than one oversized allocation plus a rehash, and `parse/corpus` re-grows per file |
| index-map pre-sizing in `rebuild_layer` (T1a.6.5.5's own subject) | **−0.7 %** | inside the drift above; the 586 allocations are per-key `Vec`s, which a map reserve does not touch |

Two of the six tasks proposed pre-sizing and both lost, which is the stage's
transferable finding: **at this scale the growth is cheaper than the estimate.**

### Start-up

`utils/e2e_baseline.py --startup`, the third row set after the milestone six
and S1a.6.4's `--blind`. Best of 15 processes:

| cell | ein.rs | ein.py CPython | ein.py PyPy |
|---|---:|---:|---:|
| `--help` | **1.02 ms** | 97.6 ms | 442.1 ms |
| `solve friends` (651 B, one rule, one fact) | **1.15 ms** | 132.3 ms | 542.4 ms |
| `saturate friends` | **1.20 ms** | 118.6 ms | 522.1 ms |

`/bin/true` through the same timing loop is **0.23 ms**, so 0.8 ms is ein's
own. The binary is 3 581 440 B, four shared libraries (`libstdc++` and
`libgcc_s` are snmalloc's), and the embedded stdlib is 67 369 B of it — 1.9 %.

**snmalloc is 0.59 ms of every process start.** The `--no-default-features`
build does `--help` in 0.43 ms and `solve friends` in 0.60 ms. That is
[§13](#13-s1a62--the-layout-stage-and-the-profile-it-starts-from)'s "0.5 ms off
`render rules zebra2`" measured on a workload that does no engine work at all,
so it is the arena set-up and nothing else. It does not change the decision —
8–16 % of a `solve` repays it inside the first 5 ms of engine work — but a
corpus cell that does nothing pays it, and the harness runs 473 of those per
tier.

### Reproducing this section

```sh
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example frontend_cost                    # the phase table + allocations
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example frontend_cost -- --rounds 3   # the diamond, counted
utils/bench_env.sh python3 utils/e2e_baseline.py --startup --runs 15
# the same-day A/B of the milestone six, two binaries in one series
git worktree add --detach /tmp/wt-prev 99fac86
(cd /tmp/wt-prev && cargo build --release --manifest-path ein.rs/Cargo.toml)
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 15 \
    --bin "prev=/tmp/wt-prev/ein.rs/target/release/ein" \
    --bin "now=ein.rs/target/release/ein"
# the parse profile: run the criterion binary itself, so perf sees one bench
BIN=$(ls -t ein.rs/target/release/deps/engine-* | grep -v '\.d$' | head -1)
CRITERION_HOME=ein.rs/target/criterion perf record -F 4999 -o /tmp/parse.perf -- \
    taskset -c 4 "$BIN" --bench --measurement-time 4 'parse/zebra2$'
perf report -i /tmp/parse.perf --stdio --no-children -F overhead,symbol | head -20
# what the comments cost: the same file parsed with them stripped (a file under
# examples/ needs a corpus entry, so this one is written and removed)
python3 -c 'import pathlib; src=pathlib.Path("examples/zebra2.ein").read_text(); \
out=[l.split(";")[0].rstrip() if chr(34) not in l else l for l in src.splitlines()]; \
pathlib.Path("examples/.tmp-nc.ein").write_text("\n".join(l for l in out if l.strip())+"\n")'
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example frontend_cost -- examples/.tmp-nc.ein examples/zebra2.ein
rm examples/.tmp-nc.ein
# the acceptance gate, from its own binary
cargo test --manifest-path ein.rs/Cargo.toml -p ein-infer --release \
    --test acceptance --no-run
time ein.rs/target/release/deps/acceptance-*[!d]
```

## 17. The boundary, measured before the stage that aims at it

**2026-08-19, `master` @ `e4d9e4e`, same machine.** The phase's one unwritten
stage has been named by every re-measurement since
[S1a.6.3](../README.md#s1a63--beta-memories-f11-d1), and this is the measurement it is written
against — taken *before* any of it is built, which is the difference between a
stage and a hope. It is [§9](#9-the-fork-entry-re-derivation)'s role for
[S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator), played for
[S1a.6.12](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot).

### The two cones, on both shapes of workload

`utils/profile_ein_rs.py`, current build, LBR cumulative:

| cone | `zebra -e` | `features/05 -e` (blind) |
|---|---:|---:|
| `Saturator::admit_from_boundary` | **37.7 %** (7.5 % self) | **28.2 %** (4.4 % self) |
| ↳ `Matcher::holds` — the guard queries themselves | 22.2 % | 18.0 % |
| ↳ the remainder — *visiting* parked candidates | **~15.5 %** | ~10.2 % |
| `Saturator::resume` — the per-entering snapshot | **10.3 %** | **12.4 %** |
| ↳ `Vec::clone⟨Entry⟩` alone, self | 3.5 % | 3.2 % |
| `btree::map::Iter::next`, self (95 % under the boundary) | 3.2 % | — |
| `Kb::facts_with`'s two iterator adapters, self | 10.8 % | 2.2 % |

**A third of the boundary is not the queries.** That split is what the stage is
built on, and neither of the two halves had been separated before: previous
sections reported `admit_from_boundary` as one number.

### What the boundary does, counted

`guard_eval` / `guard_eval_monotone` are added here — the `Saturator`'s own
`guard_evals` pair summed over **every fork of a solve**, which is what
[Q-M1a.17](../open_questions.md#q-m1a17--win-bs--80--assumed-monotone-guards-dominate)
asks for and no per-saturation field can answer.

| cell | visits | extent probes | guards asked | queries run | memo hits | monotone |
|---|---:|---:|---:|---:|---:|---:|
| `solve zebra2` | 36 943 | 73 427 | 9 978 | 9 978 | 0 | 499 (5.0 %) |
| `solve zebra2 -e` | 204 158 | 406 106 | 30 691 | 30 691 | **0** | 2 250 (**7.3 %**) |
| `solve zebra` | 41 040 | 81 768 | 8 827 | 8 645 | 182 | 906 (10.5 %) |
| `solve zebra -e` | 248 043 | 494 566 | 29 865 | 29 505 | 360 | 4 505 (**15.3 %**) |
| `solve features/05 -e` | 4 755 421 | 8 981 278 | 4 755 413 | 4 719 834 | 35 579 | 493 985 (10.5 %) |
| `solve branching/07 -e` | 0 | 0 | 0 | 0 | 0 | 0 |

Visits are `watch_stamp` (one watch stamp built per parked candidate per
round), extent probes are `watch_stamp_rel`. Three things fall out of it:

**The walk is 6.7–8.3× oversubscribed on the deep saturations.** `guard_query`
is an upper bound on candidates judged, so at most 12 % of `zebra -e`'s 248 043
visits reach a query: the other 88 % exist to build a stamp, compare it, and
discover that nothing this candidate watches has changed. `features/05 -e` is
the opposite — 1.0 visits per ask, because its 384 167 forks each judge a small
parked set once — and `branching/07 -e` is the control with **no guards at
all**, where the boundary is inert.

**The per-round guard memo answers nothing.** `guard_query − guard_eval` is its
hit count: **0** on `zebra2 -e`, 1.2 % on `zebra -e`, 0.75 % on `features/05
-e`. design/06 § Win B refinement 1 assumed two parked candidates frequently
share a guard and a projected environment; the watch stamp — refinement 2 —
filters them out before they can. The two overlap and the cheaper one wins,
which leaves a `Box<[Value]>` allocation and a hash insert per evaluation,
4.7 M of them on `features/05 -e`, for no returns.

**Q-M1a.17 is answered, against Win B.** It asked whether the exhaustive
monotone mix differs from the root-scale 11 % / 30 %. It does, in the wrong
direction: **7.3 % on `zebra2 -e` and 15.3 % on `zebra -e`**, against
design/06's ≥ 80 % projection. The structural reason the question already gave
is why scale makes it worse rather than better — a failing *monotone* guard
retires its candidate on the spot, so what stays parked is precisely what the
mechanism cannot help. The ceiling on `zebra -e` is 15.3 % × 22.2 % = **3.4 %
end-to-end**, for the headline win of the design's saturation chapter.

Root scale, for the comparison (`--example engine_cost`, unchanged since
S1a.3.4 measured it): `zebra2` 958 evaluations / 109 monotone, `zebra` 945 /
280.

### What this chooses

| task | because |
|---|---|
| [T1a.6.12.1](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot) visit only what changed | ~15.5 % of `zebra -e`, and 88 % of the visits are provably skippable — design/06 § Win B refinement 3, the one that never landed |
| [T1a.6.12.2](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot) the memo | 0–1.2 % hit rate, 4.7 M allocations on the blind cell. Free to measure, and it only gets more true after .1 |
| [T1a.6.12.5](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot) the snapshot | 10.3 % / 12.4 %, against the **0.6 %** at which [S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) declined the same change — S1a.6.3 took 4.5× off everything around it |
| [T1a.6.12.3](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot) what the queries scan | `facts_with` is 10.8 % of `zebra -e`'s self time; the join was 4.5× faster after S1a.6.3 keyed its index, and whether the guards got that is unmeasured |
| [T1a.6.12.4](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot) semi-naive guards | **last, and possibly not at all** — 3.4 % ceiling. `Matcher::holds_seeded` already exists (the lookahead uses it), so this is a wiring decision priced at its measured reach rather than a build |

### Reproducing this section

```sh
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 3 --top 14 \
    --cum-of admit_from_boundary --cum-of Saturator::resume --cum-of holds \
    solve examples/zebra.ein -e
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 2 --no-build \
    --cum-of admit_from_boundary --cum-of Saturator::resume --cum-of holds \
    --cum-of facts_with solve examples/features/05_stdlib_domain_elim.ein -e
utils/bench_env.sh cargo run --release --manifest-path ein.rs/Cargo.toml \
    --features counters -p ein-infer --example counter_cost
utils/bench_env.sh cargo run --release --manifest-path ein.rs/Cargo.toml \
    --features counters -p ein-infer --example counter_cost -- \
    examples/features/05_stdlib_domain_elim.ein examples/branching/07_lookahead_off.ein
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example engine_cost          # the root-scale mix, for the comparison
```

## 18. S1a.6.12 — the boundary, and the premise that had nothing left to bind

**2026-08-20, `master` @ `6e077b3`, same machine.** [§17](#17-the-boundary-measured-before-the-stage-that-aims-at-it)
measured the two cones this stage was written against and predicted where its
days would go. Three of the five tasks went there. The fourth went somewhere
§17 could not see, because §17 asked about the boundary's *visits* and the
answer turned out to be about its *premises*, and it is the largest single
change in the phase.

### The four targets

Best-of-7 processes, both columns measured in the same run — the "before" is
the `S1a.6.11` binary re-run today, not a recorded value, which is what makes
the deltas subtraction rather than arithmetic across two afternoons.

| workload | target | at S1a.6.11 | **at S1a.6.12** | vs PyPy ¤ |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | ≤ 200 ms | 41.1 ms | **28.9 ms** ✅ | **157×** |
| `solve zebra.ein -e` | ≤ 400 ms | 76.7 ms | **47.5 ms** ✅ | **175×** |
| `solve zebra2.ein` | — | 10.0 ms | **7.9 ms** | 320× ¶ |
| `solve zebra.ein` | — | 14.8 ms | **9.3 ms** | 327× ¶ |
| parse + load `zebra2` | ≤ 15 ms | 0.66 ms | **0.67 ms** ✅ | 642× |
| the acceptance gate (3 fixtures) | ≤ 5 s | 0.196 s | **0.127 s** ✅ | **283×** |

¤ Against the [README's PyPy column](../README.md#p1a6--performance) — 4.53 s, 8.33 s,
0.43 s, 36.0 s — the same denominator that table uses. The parse + load row is
the `load/zebra2` criterion bench and this stage does not touch the frontend:
664.1 → 672.8 µs is drift between two `cargo bench` runs, and the four frontend
rows of the bench set below show the same ±3 % in both directions.

¶ The two rows the README's table does not carry, against
[§1](#1-end-to-end-process-against-process)'s PyPy readings instead (2 529.9 ms
and 3 045.1 ms). Naming the denominator is the point: the two columns are two
runs of the same interpreter and differ by ~5 %.

The acceptance row is the fifth recorded reading of a quantity [§6](#6-cargo-bench--variance-and-the-acceptance-gate)
says has three, measured the way [§15](#15-s1a64--the-per-call-setup-and-the-enumerator-the-targets-never-run)
measures it — the three `ein-infer` acceptance tests from their own binary,
unpinned, best of three (0.135 / 0.131 / **0.127** s).

### The cells the targets do not cover

Best-of-3, same pairing:

| cell | at S1a.6.11 | **at S1a.6.12** | change |
|---|---:|---:|---:|
| `features/05 -e` | 3673.0 ms | **3010.5 ms** | **−18.0 %** |
| `features/01 -e` | 2044.3 ms | **1914.7 ms** | −6.3 % |
| `branching/07 -e` | 908.3 ms | **881.7 ms** | −2.9 % |
| `branching/06 -e` | 209.7 ms | **196.9 ms** | −6.1 % |
| `sq-bwd/houses -e` | 265.1 ms | **255.6 ms** | −3.6 % |

**No cell in either set is slower**, which is the stage's third target. The one
that moves least is `branching/07 -e` — zero guards, so the whole boundary is
inert there and what it gains is [T1a.6.12.5](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot)'s
alone. `render zebra2` reads +1.3 % (1.5 → 1.6 ms) on a cell whose own spread
is 1.3–3.2 %; it does not run the engine at all.

### The two cones the stage was written against

`utils/profile_ein_rs.py`, LBR cumulative, against §17's readings:

| cone | `zebra -e` | | `features/05 -e` | |
|---|---:|---:|---:|---:|
| | **§17** | **now** | **§17** | **now** |
| `Saturator::admit_from_boundary` | 37.7 % | **17.8 %** | 28.2 % | **24.6 %** |
| ↳ `Matcher::holds` — the queries | 22.2 % | 13.7 % | 18.0 % | 19.2 % |
| ↳ the rest — visiting parked candidates | ~15.5 % | **~4.1 %** | ~10.2 % | ~5.4 % |
| `Saturator::resume` — the snapshot | 10.3 % | **7.6 %** | 12.4 % | **7.7 %** |

Shares of a denominator that shrank by 38 % and 18 %, so the absolute numbers
are the ones to read:

| cone | `zebra -e` | `features/05 -e` |
|---|---:|---:|
| `admit_from_boundary` | 28.9 → **8.5 ms** (−71 %) | 1036 → **741 ms** (−28 %) |
| `Saturator::resume` | 7.9 → **3.6 ms** (−54 %) | 455 → **232 ms** (−49 %) |

**One of the four cone targets is met as a share** — `resume` below 6 % on
`zebra -e` was true at **5.5 %** when it was measured immediately after
T1a.6.12.5, and reads 7.6 % now because T1a.6.12.3 took another 20 % off
everything around it. `admit_from_boundary` below 28 % on `zebra -e` is met
with room; below 22 % on `features/05 -e` is not (24.6 %), for the same
reason in the other direction: that cell's boundary halved while its hypgen
and fork layers did not.

### What the boundary does now, counted

| quantity | at S1a.6.11 | **at S1a.6.12** |
|---|---:|---:|
| parked slots **copied** per round, summed over the run | 947 758 | **0** |
| boundary visits (`watch_stamp`) | 248 043 | 248 043 |
| boundary extent probes (`watch_stamp_rel`) | 494 566 | **12 864** |
| `BindingKey` hashes asked of `fired` at the boundary | 248 043 | **0** |
| guard evaluations (`guard_query`) | 29 865 | 29 865 |
| candidates offered to `unify` | 1 172 870 | **238 567** |
| ↳ of them, from guard sub-plans | 1 004 605 | **79 205** |
| premises answered by one interned lookup (`scan_ground`) | — | **148 024** |
| instructions retired, whole process | 1 019.7 M | **533.0 M** |

`guard_query` is the line that does not move, in either direction, on any cell
of the corpus: **the boundary asks exactly the same questions in exactly the
same order**, and everything above it is what asking them used to cost.

### The premise that had nothing left to bind

[T1a.6.12.3](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot)
was scoped as an instrument — split `scan_*` / `cand_*` by caller, and
optimise only if the split finds something. It found that the guards use the
index perfectly (`scan_extent_guard` is **0** everywhere) and that they own the
candidate stream:

| cell | guard scans | guard candidates | share of all candidates | per scan | ground share of guard scans |
|---|---:|---:|---:|---:|---:|
| `zebra -e` | 100 921 | 1 004 605 | **85.7 %** | 9.96 | **71.8 %** |
| `zebra2 -e` | 104 642 | 284 932 | 92.9 % | 2.72 | 72.2 % |
| `features/05 -e` | 11 131 940 | 21 504 457 | **100 %** | 1.93 | 62.0 % |
| `features/01 -e` | 599 519 | 229 143 | 35.1 % | 0.38 | **100 %** |
| `branching/07 -e` | 0 | 0 | 0 % | — | — |

A premise every one of whose slots is already bound is not a search: it asks
whether one exact proposition is in the KB. The fact store interns
propositions, so at most one fact can answer and the store names it in a hash
lookup — where the scan fetched a ten-deep participation bucket and unified
every fact in it. Identical by construction: the pattern denotes one argument
tuple, `unify` accepts a fact iff its arguments *are* that tuple, and no two
facts share one.

The two puzzles differ by 5× on what that is worth, and the table says why:
`zebra`'s guard buckets are 9.96 facts deep and `zebra2`'s are 2.72.

| cell | candidates | end-to-end |
|---|---:|---:|
| `solve zebra -e` | 1 172 870 → **238 567** | −20.6 % |
| `solve zebra` | — | **−22.3 %** |
| `solve zebra2 -e` | 306 725 → 152 996 | −4.0 % |
| `features/05 -e` | 21 504 482 → 9 979 438 | −3.6 % |
| `branching/07 -e` | 256 758 → 256 758 | +0.9 % |

`branching/07 -e` is the control that prices the check itself: it has no ground
premise anywhere, so every `rel_step` pays the slot inspection and gets
nothing. **+0.9 % at the process level, +4.0 % on the `match_hot` micro-bench**
— that is what the join path pays for the guard path's 20 %.

### Three work sets built, two reverted

design/06 § Win B refinement 3 — "index `watched relation → parked candidates`,
walk the affected ones" — was built twice and reverted twice before the third
shape landed, and the two failures are the finding:

| build | `zebra -e` instructions | visits reaching the skip test |
|---|---:|---:|
| after T1a.6.12.1a | 949 M | 248 043 |
| a per-candidate ordered work set | 963 M (**+1.5 %**) | **29 865** (exact) |
| per-guard-set chains, walked when the set moves | 1 123 M (**+18 %**) | 248 043 |
| the walk holding its own set (shipped) | **881 M (−7.2 %)** | 248 043 |

The exact work set reaches the ideal visit count and *still* loses: a park is
an ordered insert, and there are more parks than the visits they save. The
instrumentation that explained it is the one number nobody had taken —
**3 216 rounds over 947 758 parked-candidate slots, visiting 248 043 of them**,
because a round stops at its first admission, on average a quarter of the way
in. The cost was never the visits. It was copying the ordered set to walk a
quarter of it.

### Q-M1a.17, closed with the last number it needed

[T1a.6.12.4](../README.md#s1a612--the-naf-boundary-and-the-per-entering-snapshot)
— wiring `Matcher::holds_seeded` into the boundary, design/06 § Win B's
headline — is **declined**, and the ceiling is now measured on both sides of
its product:

| cell | monotone share of guard evaluations | `Matcher::holds` share of the run | **ceiling** |
|---|---:|---:|---:|
| `solve zebra -e` | 16.3 % | 13.7 % | **2.2 %** |
| `solve zebra2 -e` | 7.3 % | — | — |
| `features/05 -e` | 11.1 % | 19.2 % | **2.1 %** |
| `features/01 -e` | **100 %** | **1.4 %** | **1.4 %** |

`features/01 -e` is the cell design/06 was describing — every one of its
599 375 guard evaluations is monotone, exactly the ≥ 80 % the design assumed —
and it is also where the boundary is 2.9 % of the run. The mechanism is not
wrong about programs; it is wrong about *which* programs have a boundary worth
optimising, and no cell in the corpus has both.

### The bench set

| bench | at S1a.6.5 | **at S1a.6.12** | change |
|---|---:|---:|---:|
| `parse/corpus` | 623.0 µs | 642.1 µs | +3.1 % ‖ |
| `parse/zebra2` | 146.4 µs | 148.0 µs | +1.1 % ‖ |
| `parse/zebra2_resolve` | 509.5 µs | 518.4 µs | +1.7 % ‖ |
| `load/zebra2` | 664.1 µs | 672.8 µs | +1.3 % ‖ |
| `saturate_root/zebra2` | 1.29 ms | **1.15 ms** | −10.9 % |
| `match_hot/zebra2` | 25.0 µs | 26.0 µs | **+4.0 %** |
| `boundary/zebra` | 1.60 ms | **1.22 ms** | **−23.8 %** |
| `boundary/zebra2` | 1.28 ms | **1.18 ms** | −7.8 % |
| `fork/zebra2` | 289 ns | 284 ns | −1.7 % |
| `solve_fast/zebra2` | 7.95 ms | **5.83 ms** | **−26.7 %** |
| `solve_exhaustive/zebra2` | 38.65 ms | **26.59 ms** | **−31.2 %** |

11 benches, worst relative sd **2.37 %** (gate 3 %). ‖ The four frontend rows
are untouched by this stage and land within the ±2.8 % drift S1a.6.5 recorded
between two `cargo bench` runs. `match_hot` is the exception that is *not*
drift: it is a join-only micro-bench and the ground-premise check costs it one
slot inspection per premise, which is the same +0.9 % `branching/07 -e` shows
at process scale.

### Where `zebra -e` stands

47.5 ms, from 76.7 at the start of the stage and 585.8 at the start of the
phase — **12.3×** across P1a.6, and 175× the PyPy column. The profile that
chooses what comes next no longer has a block above 8 %:

| block | self |
|---|---:|
| `Matcher::walk` | 7.2 % |
| `Matcher::ground_args` | 4.8 % |
| `Saturator::enqueue_pass` | 4.7 % |
| `Kb::contains` | 4.4 % |
| `sn_rust_dealloc` | 4.2 % |
| `Matcher::unify` | 4.0 % |
| `Saturator::enqueue_binding` | 3.7 % |
| `FactStore::find` | 3.6 % |

Two of the top eight are the ground path's own — `ground_args` builds the tuple
and `FactStore::find` looks it up, 8.4 % between them to remove 79.7 % of the
candidates. The enqueue path (`enqueue_pass` + `enqueue_binding` + the
`BindingKey` hashing under them) is now as large as the matcher's, and that is
what [S1a.6.7](../README.md#s1a67--re-measure-the-lever-matrix) re-levers against.

### Reproducing this section

```sh
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7 \
    --bin before=<a 6e077b3^^^^^ build> --bin after=ein.rs/target/release/ein
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 3 --blind \
    --bin before=<same> --bin after=ein.rs/target/release/ein
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 4 \
    --cum-of admit_from_boundary --cum-of Saturator::resume --cum-of holds \
    --cum-of ground_args solve examples/zebra.ein -e
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 2 --no-build \
    --cum-of admit_from_boundary --cum-of Saturator::resume --cum-of holds \
    solve examples/features/05_stdlib_domain_elim.ein -e
utils/bench_env.sh cargo run --release --manifest-path ein.rs/Cargo.toml \
    --features counters -p ein-infer --example counter_cost
utils/bench_env.sh cargo run --release --manifest-path ein.rs/Cargo.toml \
    --features counters -p ein-infer --example counter_cost -- \
    examples/features/05_stdlib_domain_elim.ein \
    examples/features/01_not_and_absent.ein \
    examples/branching/07_lookahead_off.ein
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
python3 utils/criterion_table.py --max-rsd 3 --json ein.rs/bench-out/criterion.json
```

## 19. S1a.6.7 and S1a.6.6 — the lever matrix in two engines, and the fuzzer

**2026-08-20, `master` @ `62381ba`, same machine.** The phase's last two
stages ship **no engine change between them** — one re-measures, one
generates — so the four targets below are a *confirmation* rather than a
delta, and the interesting numbers are what the two instruments said about
things the phase had been assuming.

### The four targets, at the close

Best-of-7 processes, `utils/bench_env.sh`, pinned:

| workload | target | at S1a.6.12 | **at the close** | spread |
|---|---:|---:|---:|---:|
| `solve zebra2.ein -e` | ≤ 200 ms | 28.9 ms | **29.0 ms** ✅ | 1.4 % |
| `solve zebra.ein -e` | ≤ 400 ms | 47.5 ms | **47.2 ms** ✅ | 1.1 % |
| `solve zebra2.ein` | — | 7.9 ms | **7.8 ms** | 2.2 % |
| `solve zebra.ein` | — | 9.3 ms | **9.3 ms** | 4.4 % |
| parse + load `zebra2` | ≤ 15 ms | 0.67 ms | **0.67 ms** ✅ | — |
| the acceptance gate | ≤ 5 s | 0.127 s | **0.127 s** ✅ | — |

Peak RSS 17.3 MB on every row. The last two are §18's, unchanged by
construction: nothing between `6e077b3` and here touches the engine.

### What the lever matrix says now — and what it says about itself

[`utils/feature_matrix.py`](../../../../utils/feature_matrix.py) drove ein.py
in-process from S1.20.I3 until this stage; it now drives **both engines as
processes**, delivers each lever through a generated `(config …)` block (five
of the ten have no CLI flag), and reads the verdict and twelve counters back
out of `--json-summary` — the T0/T1 surface itself. **Every cell of every
matrix agrees between the engines** on the verdict, `k`, the goal bindings and
those counters: 22 cells on `zebra2`, 22 on `zebra`, 22 on `lattice/02`, 6 on
`branching/06`, with one exemption where ein.py stops on its 90 s budget and
ein.rs does not.

Two method changes were forced by the first run, and the second is the finding:

| | what it was | what it is |
|---|---|---|
| order | one cell at a time, all its runs | **round-robin over the cells** |
| resolution | asserted | a **`control` cell** — byte-identical to `baseline`, measured last |

Measured cell-by-cell, PyPy's baseline — the divisor of every ratio in the
table — runs first and reads ~20 % fast, which came out as a uniform 1.2×
tax on eight levers that ein.rs measured at exactly 1.0× with identical
entering counts. And with round-robin in place, the control still reads:

| | ein.py (PyPy) | ein.rs |
|---|---:|---:|
| `zebra2` fast | 1.0× | 1.0× |
| `zebra2` exhaustive | **1.2×** | **1.0×** |
| `zebra` fast | 0.9× | 1.1× ‖ |
| `zebra` exhaustive | 1.0× | 1.0× |

‖ 8 ms against 7 — the resolution of an integer-millisecond column, not drift.

**So the ein.py column cannot resolve anything below ~1.2×, and the ein.rs
column resolves 1.0× exactly.** Four rows the 2026-08-17 table reported
between 0.9× and 1.1× were never measurements. The two levers it named
survive, and one of them grew:

| lever off | `zebra2 -e` ein.py | ein.rs | `zebra -e` ein.py | ein.rs |
|---|---:|---:|---:|---:|
| `enable_singleton_writeback` | **∞** (3 358 @ 90 s) | **56.6×** (3 831 enterings) | **∞** (1 277 @ 90 s) | **43.8×** (3 834) |
| `enable_fail_fast_fork` | **2.4×** | **7.1×** | **2.9×** | **7.0×** |
| `lattice_order="score-sum"` | 1.0× (134) | 1.2× (134) | **0.6×** (62) | **0.6×** (62) |
| `enable_pre_branch_lookahead` | 1.0× (111) | 1.0× (111) | 1.1× (134) | 1.1× (134) |
| the other five | 1.0–1.1× | **1.0×** | 1.0× | **1.0×** |

`enable_fail_fast_fork` is the phase's own result seen from the other side:
what it removes is a fixed quantity of dead-fork saturation, everything around
it shrank, so its ratio **rose** from 1.9× (ein.py, 2026-08-17) to 7.1×
(ein.rs, today) — **86 %** of an exhaustive `zebra2` without it is saturating
forks already known to be dead.

### The lever that is not a prune

[T1a.6.7.3](../README.md#s1a67--re-measure-the-lever-matrix)
asked for the lookahead on a deeper puzzle. Two corpus fixtures answer, and
the answer is not a ratio:

| fixture | lookahead on | off |
|---|---|---|
| `branching/06` fast | Solution k=1, 67 enterings, **2 ms** | **Contradiction k=0**, 11 501, **896 ms** (448×) |
| `branching/06 -e` | Ambiguity k=22, 5 173, 197 ms | **Contradiction k=0**, 11 501, 890 ms |
| `lattice/02 -e` | Ambiguity k=3, 6 | **Contradiction k=0**, 7 |

Both engines, to the digit. `complete(kb)` is "the generator proposes nothing
undecided" and the generator's candidates are **lookahead-filtered**, so a
candidate the one-step simulation kills is *decided* with the lever on and
*open* with it off. With it off, `branching/06` reports `Contradiction` on a
puzzle with 22 models. It is not a port bug and not a prune; it is a
definition, and it is parked as [F4 Q40](../../../../plans/followups/f4_cross_cutting.md).
Not the kill *cache*: with `-K` the verdict is unchanged (5 192 enterings
against 5 173).

`lattice_order="score-sum"` at **0.6× on `zebra` and 1.2× on `zebra2`**, in
both engines, is a candidate default change recorded with its own
counter-example and left as a decision — Rule "resist changing a default in
the same commit as the measurement", and here in the same *stage*.

### The fuzzer, and the four things it found

S1a.6.6 ran for the first time on the same day. One session of **21.3
minutes, 12 080 cases** at 20 jobs — 567–700 cases/min, **85.2 %** of them
loading (the stage asked for ≥ 80 %), the rest rejected by the frontend, which
is material rather than waste: what two engines must agree on there is the
*message*. Eight cells reported, in three classes and no fourth.

The first ten minutes produced 13 reported cells, and the triage is the point:

| what | how many | outcome |
|---|---:|---|
| **an integer goal binding** | 4 | **T0 bug in ein.rs** — `"y": "8"` where ein.py writes `8`. Fixed; `Json::BigInt` carries the IR's unbounded `INT` |
| **a nested-fact goal binding** | 3 | **bug in ein.py** — `json.dumps` raised and wrote no summary. Fixed; renders the s-expression, as ein.rs already did |
| **D2's second shape** | 5 | `sorted(alive)` over two `Fact` args. **No mixed types needed** — one `(hrule … :assert (not …))`. Accepted, with a fixture and a re-stated ledger entry |
| **a crash-parity cell** | 1 | `(?R ?x)` unbound: identical class, identical exit code, only ein.py's traceback wrapper. **Passes** — a corpus entry, not a divergence |

Two more bugs landed in the same session and are worth the same table: an
unbound `:assert` variable ended ein.py's traceback with `KeyError: "…"` —
whose `str` is the *repr* of its key — where ein.rs printed the message bare
(the case [Q-M1a.14](../open_questions.md#q-m1a14--crash-parity) named in its
first paragraph and nothing had ever reached), and `(query :goal (?R Rex
Animal))` — two lines — is a program ein.py rejects **inside its table
renderer** and ein.rs ran to completion.

The second of those is also where the phase's own gate earned its keep. The
first fix checked the goal in the CLI, before the verdict was known, and
turned one divergence into another: ein.py raises only when it *renders a
solution block*, so a contradictory puzzle with the same broken goal exits 0
on both sides and the eager check made ein.rs exit 1. `trace_parity` — which
runs the renderer, not the CLI — reported it within the hour, and both arms
are fixtures now (`query-goal-free-head.ein`, `…-unsat.ein`).

Two of those are genuine parity bugs in a surface **five phases of byte parity
had signed off**, and the reason both hid is the same: `stdout` is identical
on all of them, and no corpus puzzle binds a query variable to anything but a
symbol. That is precisely the gap
[design/01 §7](../design/01_parity_contract.md) predicted a fuzzer would
cover.

Three fuzzer bugs were found by the fuzzer's own controls, and they are worth
recording because each is a way to *look* successful:

1. **A canary that itself diverges** made every minimisation shrink to the
   first form that parses — `still_diverges` accepted any reported cell, not
   the case's own. Now it checks the path, and a diverging canary stops the
   run.
2. **A crash reported at T3 is not a finding.** Judged in the `crash-parity`
   group — exit code + exception class — the `unbound-relation-head` shape
   passes, and it was being re-found on every batch.
3. **A generator that produces the ledger's own entry** reports the known
   answer forever: four findings in five were D2 until `(hrule …)` stopped
   getting negative heads and int arguments, and the two D2 fixtures left the
   mutation seed set.

After all three, the same 120 cases produce **0 findings and 4 crash-parity
candidates**.

### D1 and D2, re-priced at the close

[F11](../../../../plans/followups/f11_deductive_layer_perf.md) named this milestone as
its own most likely promotion trigger. Neither entry landed, and D1's number
moved twice more in the same direction:

| when | candidates | steps entered (`walk`) | per step |
|---|---:|---:|---:|
| before T1a.6.3.0 | 25 160 149 | 530 405 | **47.4** |
| after it | 1 171 385 | 530 405 | **2.21** |
| **at the close** | **238 567** | 532 115 | **0.45** |

`solve zebra -e`. The step count is flat to 0.3 % across all three rows, which
is what makes them comparable: the matcher takes the same decisions and looks
at 100× fewer facts to take them. A beta-memory materialising less than one
tuple per step, behind a per-fork table measured at **+7.6 %**, is not a
lever. D2's cost half moved *away* from its trigger: the matcher's five hot
functions are about a fifth of a 47 ms solve, where the phase began at 66.9 %.

### The gate, at the close

`./run_tests.sh` — **1 516 pytest + 21 acceptance + the whole ein.rs
workspace, green** — and the corpus-wide parity run:

```text
group               same    DIFF    skip
crash-parity          11       2       0
load-negative         87       0       0
parse-negative        12       0       0
positive             358       0       0
stdlib                28       0       0
total                496       2       0     tier T3, 179.4s of engine time
```

The two are D2's two shapes. Seven of those cells are new this phase and all
seven are the fuzzer's: two regressions for the bugs it found, four
`crash-parity` entries, and one that exists only to pin an asymmetry — the
`Contradiction` arm of `query-goal-free-head`, which exits **0** where its
sibling exits 1 because no solution block is rendered and so the goal is never
compiled. That distinction is not decoration: the first fix for the sibling
missed it and traded one divergence for another, and `trace_parity` — which
runs the renderer rather than the binary — caught it inside the hour.

### Reproducing this section

```sh
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7
utils/bench_env.sh python3 utils/feature_matrix.py --runs 5 \
    --python .venv-pypy/bin/python
utils/bench_env.sh python3 utils/feature_matrix.py --runs 3 \
    --python .venv-pypy/bin/python --puzzle examples/zebra.ein
utils/bench_env.sh python3 utils/feature_matrix.py --runs 3 \
    --python .venv-pypy/bin/python --cells lookahead \
    --puzzle examples/branching/06_lookahead_on.ein
utils/bench_env.sh python3 utils/feature_matrix.py --runs 5 \
    --python .venv-pypy/bin/python \
    --puzzle examples/lattice/02_genuine_3set_death.ein
utils/bench_env.sh cargo run --release --manifest-path ein.rs/Cargo.toml \
    --features counters -p ein-infer --example counter_cost
# the fuzzer: a session, and its own negative control
python3 utils/fuzz_ein.py --minutes 150 --batch 80 --jobs 20
python3 utils/fuzz_ein.py --iters 2 --batch 2 --tier T2 --canary "" \
    --impl-b "python3 utils/mutant_ein.py ein.rs/target/release/ein"
```

## Reproducing all of it

Every line from the repo root, every measurement through the fingerprint.
**This block is maintained**; the per-section ones above are records. What it
cannot reproduce is listed in the note at the top of the file — the CPython and
PyPy columns and the four parity tiers, whose instruments left with the second
engine at [P1a.10](../README.md#p1a10--one-implementation).

```sh
utils/bench_env.sh --report                    # the machine state, nothing else
mkdir -p ein.rs/bench-out                      # git-ignored artefact root

# §1 processes. §2's Python halves are frozen: `utils/bench_baseline.py` is
# gone and `cargo bench` below is the whole in-process set.
utils/bench_env.sh python3 utils/e2e_baseline.py --json ein.rs/bench-out/e2e.json

# §3 attribution (add `--callers SUBSTR` for a caller breakdown)
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 \
    --cum-of ein_infer::compile --json ein.rs/bench-out/prof-zebra2-e.json \
    solve examples/zebra2.ein -e

# §4 work counters. The ein.py column was `utils/count_work.py`, which wrapped
# module attributes on an engine that is gone; this half survives.
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example counter_cost

# §15 the per-call hypgen table, and the blind-enumerator cells
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example hypgen_calls
utils/bench_env.sh python3 utils/e2e_baseline.py --blind --runs 5

# §19 the lever matrix, with the control row that prices the method
utils/bench_env.sh python3 utils/feature_matrix.py --runs 5
# §19 the fuzzer — a session over the properties one engine can check
python3 utils/fuzz_ein.py --minutes 150 --batch 80

# §17 / §18 the boundary and the snapshot — the two cones, and what they do
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 3 \
    --cum-of admit_from_boundary --cum-of Saturator::resume --cum-of holds \
    --cum-of ground_args solve examples/zebra.ein -e

# §16 the load path, phase by phase, and what a process pays before it starts
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer \
    --example frontend_cost
cargo run --release --manifest-path ein.rs/Cargo.toml --features counters \
    -p ein-infer --example frontend_cost -- --rounds 3
utils/bench_env.sh python3 utils/e2e_baseline.py --startup --runs 15

# §5 memory, and §13's distributions (arity, extents, plan width, fork depth)
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example alloc_cost
cargo run --release --manifest-path ein.rs/Cargo.toml -p ein-infer --example layout_shape

# §9 the fork-entry split — re-run at the end of every stage in the phase
python3 utils/fork_split.py --json ein.rs/bench-out/fork-split.json

# §11 the resumed fork saturator — a build of its own, off on every ship path
cargo build --release --manifest-path ein.rs/Cargo.toml \
    --features fork-delta --target-dir ein.rs/target-fd
python3 utils/fork_delta_verify.py --json ein.rs/bench-out/fork-delta.json

# §6 the bench set and its variance gate (§13 adds the system-allocator arm:
# the same benches with `-p ein-corpus --no-default-features`)
utils/bench_env.sh cargo bench --manifest-path ein.rs/Cargo.toml
python3 utils/criterion_table.py --max-rsd 3 --json ein.rs/bench-out/criterion.json

# §12 the parity contract. The two tiers and the PYTHONHASHSEED sweep needed
# two operands and have none; what asks their questions of the engine that is
# left is `cargo test`, and the successors are named in the ledger:
#   the tiers            -> tests/golden/corpus_exits.txt + corpus_shapes.md5
#   the determinism sweep -> EIN_ID_SEEDS=8 cargo test -p ein-render \
#                                --test id_order_invariance
#   the mutant control    -> cargo test -p ein-infer --test event_cut_control
EIN_ID_SEEDS=8 cargo test --manifest-path ein.rs/Cargo.toml \
    -p ein-render --test id_order_invariance

# the gate this all has to leave green
cargo test --manifest-path ein.rs/Cargo.toml --workspace
```
