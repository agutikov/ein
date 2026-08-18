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
| — | ein.rs P1a.6 (optimised, `--jobs 1`) | — | — | — | target ≤ 0.2 s / ≤ 0.4 s / ≤ 5 s |
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
