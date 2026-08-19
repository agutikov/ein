# M1a design docs — ein.rs

The *how* of the [M1a Rust port](../README.md). The milestone README
carries scope, phases and status; these eleven documents carry the
decisions. (`09 — Server mode` was **deleted 2026-08-18** when the server
was dropped; the numbering keeps its gap, like a closed question id. It
is in git history.)

## Reading order

Three groups, and they are worth reading in this order because each
constrains the next:

1. **The contract** — what may not change.
   [01 Parity contract](01_parity_contract.md) →
   [02 Determinism & order](02_determinism_and_order.md) →
   [11 Shared assets](11_shared_assets.md)

2. **The machine** — what changes underneath.
   [03 Data model](03_data_model.md) →
   [04 IR frontend](04_ir_frontend.md) →
   [05 Matcher](05_matcher.md) →
   [06 Saturation](06_saturation.md) →
   [07 Search layer](07_search_layer.md)

3. **The scale-out** — what the port unlocks.
   [08 Parallelism](08_parallelism.md) →
   [10 Binary format](10_binary_format.md) →
   [12 Toolchain & layout](12_toolchain_and_layout.md)

Group 1 is not optional preamble. Every optimisation in group 2 is
justified *only* because group 1 can prove it changed nothing; a reader
who skips to [05](05_matcher.md) will find claims like "this preserves
match order" whose enforcement lives in [02](02_determinism_and_order.md).

## What each doc settles

| doc | settles | phase |
|---|---|---|
| [01 Parity contract](01_parity_contract.md) | the four parity tiers, the JSONL oracle event protocol, the corpus, the divergence ledger | [P1a.0](../p1a.0_conformance_harness/README.md) |
| [02 Determinism & order](02_determinism_and_order.md) | the audited list of order-sensitive sites in ein.py and the Rust structure that reproduces each | [P1a.0](../p1a.0_conformance_harness/README.md)–[P1a.5](../p1a.5_presentation/README.md) |
| [03 Data model](03_data_model.md) | `Symbol`/`Value`/`FactId` as `u32`, the fact row store, the seven indexes, the layered COW KB, arenas | [P1a.2](../p1a.2_kb_core/README.md) |
| [04 IR frontend](04_ir_frontend.md) | hand-written lexer + recursive-descent parser, AST arena, dumper, macro expansion, import resolution | [P1a.1](../p1a.1_ir_frontend/README.md) |
| [05 Matcher](05_matcher.md) | plan bytecode, slot registers + backtrack trail, candidate selection, beta-memories, WCOJ trigger | [P1a.3](../p1a.3_deductive_core/README.md), [P1a.6](../p1a.6_performance/README.md) |
| [06 Saturation](06_saturation.md) | the two-phase closure/boundary loop, semi-naive delta, the two heaps, incremental NAF invalidation, the native mirror | [P1a.3](../p1a.3_deductive_core/README.md) |
| [07 Search layer](07_search_layer.md) | hypgen enumeration, lookahead, apriori generation, no-good store, the layer loop, verdict synthesis | [P1a.4](../p1a.4_search_layer/README.md) |
| [08 Parallelism](08_parallelism.md) | four parallel levels; speculate-and-validate with read-set tracking; the `--jobs` contract | [P1a.7](../p1a.7_parallelism/README.md) |
| [10 Binary format](10_binary_format.md) | `.einb` container layout, mmap-ability, versioning, content addressing, the solution store | [P1a.8](../p1a.8_binary_container/README.md) |
| [11 Shared assets](11_shared_assets.md) | repo-root `stdlib/`, resolution order in both impls, drift detection, the shared corpus | [P1a.0](../p1a.0_conformance_harness/README.md) |
| [12 Toolchain & layout](12_toolchain_and_layout.md) | the `ein.rs/` workspace, crate split, dependency policy, MSRV, CI, benches | [P1a.0](../p1a.0_conformance_harness/README.md) |

## Measured

Kept here so the milestone's claims stay falsifiable. Filled in per
phase; the baseline row is the promotion-time measurement from the
[milestone README](../README.md#baseline--what-einrs-has-to-beat).

Refreshed by one command per implementation, producing the same
measurement set (§4 of [12](12_toolchain_and_layout.md)) so the columns
are comparable rather than merely adjacent:

```sh
python3 utils/bench_baseline.py --json /tmp/py.json   # or .venv-pypy/bin/python
cd ein.rs && cargo bench                              # from P1a.6
```

`EIN_SRC=<other-checkout>/ein.py/src` points the Python benches at a
different revision, which is how a before/after is taken without
stashing the tree under test.

[S1a.6.1](../p1a.6_performance/s1a.6.1_profile_baseline.md) added five more,
because a wall-clock pair cannot say *why* or *whether the two engines did
the same work* — every one of them runs under `utils/bench_env.sh`, which
prints the machine state the numbers were taken under and pins to a P-core:

```sh
utils/bench_env.sh python3 utils/e2e_baseline.py     # processes, not calls
utils/bench_env.sh python3 utils/profile_ein_rs.py --repeat 10 solve examples/zebra2.ein -e
utils/bench_env.sh python3 utils/count_work.py       # ein.py's work counters
python3 utils/criterion_table.py --max-rsd 3         # criterion's sd, gated
cd ein.rs
cargo run --release --features counters -p ein-infer --example counter_cost
cargo run --release -p ein-infer --example alloc_cost
```

The tables they produce live in
[p1a.6_performance/baseline.md](../p1a.6_performance/baseline.md); the raw
artefacts go to `ein.rs/bench-out/` (git-ignored, machine-specific — the same
split `utils/feature_matrix_results.json` uses).

| date | build | `zebra2 -e` e2e | `zebra -e` e2e | acceptance gate | note |
|---|---|---|---|---|---|
| 2026-08-17 | ein.py, CPython 3.14 | 5.69 s | — | — | baseline |
| 2026-08-17 | ein.py, PyPy 3.11 | 4.07 s | 8.15 s | **43.7 s** ‡ | baseline |
| 2026-08-18 | ein.rs P1a.1 (frontend only) | — | — | — | `parse`: **758 µs** vs 760.6 ms CPython / 230.9 ms PyPy (1 003× / 305×). zebra2 parse + resolve + expand: **824 µs** vs 618.9 ms / 193.7 ms |
| 2026-08-18 | ein.rs P1a.2 (KB core) | — | — | — | `load` zebra2 (parse + imports + macros + index build): **1.03 ms** vs 625.6 ms CPython (607×). `fork` + first delta write: **248 ns** vs 17.3 µs (70×). Peak RSS on `load(zebra2)`: **3.1 MB** vs 46.6 MB (15×); the load itself adds **0.73 MB** vs 16.2 MB (22×) |
| 2026-08-18 | ein.rs P1a.3 (deductive core) | — | — | — | root saturation of zebra2 (load excluded): **2.89 ms** vs 90 ms CPython (31×). `match_hot`, every plan over the saturated zebra2 root: **38.6 µs** vs 2 110 µs (55×), over the same 2 075 premises. Compiling zebra2's 19 plans: **21.8 µs**. `boundary`, a zebra root saturation: 7.10 ms, of which **80 %** is the boundary |
| 2026-08-18 | ein.rs P1a.4 (search layer) | **194 ms** § | **587 ms** § | **0.87 s** ¶ | `solve_fast` zebra2 (11 enterings): **43.0 ms** vs 1.22 s CPython (28×). `solve_exhaustive` (101): **194 ms** vs 5.00 s (26×). `solve zebra` fast (13): **119 ms** vs 6.99 s (59×); exhaustive (111): **587 ms** vs 30.4 s (52×). One hypgen pass, zebra2 hrule + lookahead: **656 µs** vs 18.3 ms (28×) |
| 2026-08-18 | ein.rs P1a.5 (parity, unoptimised) | **198.8 ms** | **585.8 ms** | **1.27 s** ◊ | the S1a.6.1 baseline, *process* measurements: 24.8× and 15.0× against the same day's PyPy (4.94 s / 8.79 s). Peak RSS 17.4 MB vs 223.1 MB. `boundary` zebra2 **2.79 ms** vs 66.2 ms PyPy (23.7×); `solve_exhaustive` 198.14 ms ± 1.41 %, and every one of the 11 criterion cases under the 3 % gate. Attribution, `zebra2 -e`: saturate 59.7 %, match/bind 29.0 %, hypgen 7.3 % — but `zebra -e` is match/bind **66.9 %**. Work counters agree with ein.py **exactly** on `plan_compile` / `fact_insert` / `guard_query` / `watch_stamp` / `watch_stamp_rel` and within 1.6 % on `unify_slot` / `candidates` / `walk` |
| 2026-08-18 | ein.rs P1a.6 / S1a.6.8 | **138.1 ms** | **539.9 ms** | **1.02 s** | design/06 § Win A finally built — the plan memo is per **run**, not per fork: `plan_compile` 17 430 → **305**, `ein_infer::compile` 21.1 % → **2.4 %** cumulative, and half the run's allocations went with it (2 536 702 → 1 344 404). `Kb::n_facts_of` maintains a per-relation count instead of folding over the layer stack: 9.5 % → **1.2 %** self, off `zebra -e`'s top-20 entirely. `boundary` zebra2 2.79 → **2.71 ms**, zebra 7.32 → **7.25 ms**; `saturate_root` 2.76 → **2.70 ms**; `solve_exhaustive` 198.14 → **133.10 ms**; **`fork` 257 → 268.9 ns**, the price of cloning the count map per fork. The two halves move different puzzles — the memo is 18.3 % of `zebra2 -e` and 0.1 % of `zebra -e`, the extent count 13.9 % and 7.3 % — so they were built and measured separately. T3 472/473, D2 only; the verbose event stream byte-identical |
| 2026-08-19 | ein.rs P1a.6 / S1a.6.9 | **99.1 ms** | **397.2 ms** | — | **all four targets met.** The fork resumes root's saturation instead of re-deriving it (`Saturator::resume`): fork firings 38 136 → **9 834** on `zebra2 -e` and 113 746 → **26 656** on `zebra -e`, fork compiles → **0**, and `solve zebra.ein -e` 539.9 → **397.2 ms** against a ≤ 400 ms target — the milestone's last unmet one. Verified over **3 228 853** enterings of the whole corpus, compared fact by fact and justification by justification: the verdict, `k`, the models, the printed unsat core, the entering count, each entering's `kind`, every alive fork's fixpoint and **all 85 `summary.json` fields — T0 and T1 in full** — are identical. What is not: T2 (−62.5 % / −75.9 % at `verbose`, and −58.8 % / −74.2 % at `normal`), T3's firing lists on the seven entries that render one, and the **primary justification of 267 529 facts**, which pick a different equally valid derivation because a resumed fork inherits root's parked candidates with root's tiebreakers. ein.py is unchanged; [D3](../divergences.md) records the divergence and [Q-M1a.18](../open_questions.md) the decision. `--trace` gained a *Before any assumption* section, without which the solution's proof silently lost every rule that fires only at root. The `Arc`-layered snapshot was **not** built: `perf` puts the deep copy at 0.6 %, and the matcher at **80.5 %** |
| 2026-08-19 | ein.rs P1a.6 / S1a.6.2 T1a.6.2.7 | **83.0 ms** | **366.1 ms** | 0.58 s | the global allocator, chosen by measuring three: `snmalloc` −15.9 % / −7.5 % end-to-end against glibc `malloc`, at **+1.2 MB** peak RSS where `mimalloc` matched the speed and cost **+7.2 MB** (+42 %) and `tikv-jemallocator` returned two thirds of the win and does not build on MSVC. In process: `solve_exhaustive` 95.36 → **79.89 ms**, `saturate_root` and `boundary/zebra2` **−22 %**, `load` −12.5 %, **`match_hot` −0.6 %** — the control, a bench that allocates nothing on its timed path — and **`fork` +11.7 %**, a fresh arena's slow path. Allocator self time 20.0 → **9.0 %** on `zebra2 -e` and 9.4 → **3.0 %** on `zebra -e`, which is now **86.5 % match/bind**, 83.3 % of it in five functions. Allocation counts, every work counter, T3 472/473 and the acceptance gate all unchanged — `alloc_cost` counts through `System` on purpose, so it measures the program. `--no-default-features` is the escape hatch. Caught in passing: `debug = 1` on the `profiling` profile made `cmake`-rs build the vendored allocator as `RelWithDebInfo`, and the profiling binary ran **+49.6 %** slower than release until two package-scoped overrides fixed it |
| 2026-08-19 | ein.rs P1a.6 / S1a.6.2 | **75.8 ms** | **349.1 ms** | 0.58 s | the layout stage, and **five of its eight tasks were closed by measurement rather than by code**. What shipped: a 20-byte `Row` holding two arguments inline (−8.5 % / −4.7 %) — the row got *bigger*, because the store is 22 KB and has never left L1, so the cost was a two-load dependency chain and not cache footprint. It only pays with the caller change next to it: `FactStore::row` + `args_of` read the row once, where `rel`-then-`args` loads it twice and `get` resolves arguments **79 %** of `zebra2`'s candidates never read. What did not: bucket-major storage and the index key, because **99.1 %** of `zebra -e`'s candidates come from a full extent scan and 0.9 % from a bucket; `SmallVec` sizing, because the hot path allocates nothing; and the delta-flatten threshold, which was **built and reverted at +7.6 %** — a flat extent index is 8 % *faster* on `match_hot` and 7.6 % slower on the search, because a fork shares its parent's index behind an `Arc` and flattening gives each of 24 live KBs a private copy. In process: `solve_exhaustive` 79.89 → **73.51 ms**, `solve_fast` 20.12 → **18.21 ms**, `match_hot` 38.1 → **35.3 µs**, `boundary/zebra` 6.69 → **6.37 ms**. Every work counter identical; T3 472/473, D2 only |
| 2026-08-19 | ein.rs P1a.6 / S1a.6.3 | **44.0 ms** | **78.1 ms** | **0.28 s** | the stage that was going to build beta-memories fixed the **alpha**-memory instead and then closed its own gate. `index_fact` keyed only non-nested arguments, so a `(not (R ?b ?i))` premise — `stdlib/slots.ein`'s, and **99.1 %** of an exhaustive `zebra`'s candidates — walked a 368-fact extent; the key now reaches one level *inside* a nested argument. `candidates` 25 160 149 → **1 171 385**, `unify_slot` 51.5 M → **3.5 M**, and **every counter that measures a decision — `walk`, `plan_run`, `binding_key`, `fact_insert`, `guard_query`, `watch_stamp`, `fork`, enterings — identical to the digit**, which is what a narrowing means. T2 **239/240**, T3 **472/473**, D2 the only cell. Then a 2048-bit Bloom filter per layer, because with the lookup now the common case a fork 24 layers deep spent 15.6 % of the run hashing one key per layer: −7.3 %, sized by sweep (512 → −6.0 %, 8192 → −7.2 %). In process: `solve_exhaustive` 73.51 → **40.17 ms**, `boundary/zebra` 6.37 → **1.65 ms (3.9×)**, `match_hot` 35.3 → **24.4 µs**, `solve_fast` 18.21 → **8.41 ms**. **The beta-memory was not built**: the intermediate it materialises is now **2.2 tuples per step entered** (47.4 before), and T1a.6.2.5 measured a per-fork copy of that shape at +7.6 % — F11 D1 re-priced, Q-M1a.10 answered *no*, D2's cyclic body found in `std.slots` with its cost half still unmet. Per-fork delta 3.9 → 6.2 KB |
| — | ein.rs P1a.6 (optimised, `--jobs 1`) | — | — | — | target ≤ 0.2 s / ≤ 0.4 s / ≤ 5 s — **all four met**, with 80 % of headroom on the tightest and both `-e` cells at **112× PyPy**; `zebra -e` is **37.7 % match/bind**, and what is left is the NAF boundary (~10.4 %) and the allocator (~12 %) |
| — | ein.rs P1a.7 (`--jobs 8`) | — | — | — | — |

† The `zebra2 -e` / `zebra -e` figures were measured 2026-08-17 on the
dev machine.

§ **Search only, not end-to-end** — the baseline rows are whole runs, and
the milestone's own attribution puts 200 ms of parse and 430 ms of load
inside `solve zebra2 -e`'s 5.69 s. Those have their own rows above, at
1 003× and 607×, so folding them in here would flatter this one with two
others' results. End-to-end the port is ~195 ms against PyPy's 4.07 s —
**21×**, the ≥ 20× target met.

¶ **Not the same acceptance gate as the rows above it.** ein.py's is 21
tests, 43.7 s under PyPy; this is the three P1a.4 fixtures
(`test_zebra_two_ontologies` / `test_zebra_three_classes` /
`test_mode_consistency`) re-expressed as
`ein.rs/crates/ein-infer/tests/acceptance.rs`. The rest of that gate is
CLI and trace surface, which is
[P1a.5](../p1a.5_presentation/README.md)'s; this row will grow to the
whole thing there.

◊ **The three fixtures, not the 21-test gate** — see ¶. ein.py's three
fixtures are 36.0 s under PyPy on the same day, so 28×; the whole
`acceptance/` gate is 49.3 s, the *third* recorded value of that number after
~91 s (S1.21.8) and 43.7 s (P1a.0), with nothing in ein.py changed between
them.

‡ **Re-measured at P1a.0, and it moved.** The milestone README carried
~91 s from S1.21.8; `./run_tests.sh --acceptance-only` on the dev machine
is 43.7 s for the 21 acceptance tests. Machine differences account for
some of it and S1.9.E23's fail-fast fork saturation — which landed after
that recording and removed ~64 % of dead-fork saturation time — for the
rest. Recorded rather than reconciled: what the target is measured
against has to be a number someone took, not one someone remembered.

## Conventions used in these docs

- **Rust snippets are sketches, not committed API.** They exist to pin
  a *shape* (how many bytes, how many indirections, what is copied on a
  fork). Names will drift; the byte counts must not.
- **Every "faster" claim names the ein.py site it replaces**, so the
  parity harness knows what to watch.
- **`§Ox`** refers to the operation numbering in
  [`docs/kernel/inference/architecture_and_algorithms.md`](../../../docs/kernel/inference/architecture_and_algorithms.md)
  §4–§6 (O1 join, O2 saturation, O3 NAF, O4 equality, O5 clash,
  O6 provenance, O7 lattice, O8 pruning, O9 canonicalisation). That
  document is the shared vocabulary between ein.py and ein.rs; these
  docs extend it rather than restate it.
