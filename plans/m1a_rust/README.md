# M1a — Rust port (ein.rs)

**Estimate:** ~7 months focused — 53 stages, ~31 weeks of stage
estimates (parity gate at ~week 17).
**Status:** **in progress** — promoted from placeholder 2026-08-17 with the
scope decision made (see § The decision); [P1a.0](p1a.0_conformance_harness/README.md)
shipped the same day. Slotted between M1 and M1b.
**Scope change 2026-08-18:** server mode is **dropped** — see
§ Non-goals; P1a.8 keeps only the `.einb` container.
**Depends on:** M1 (**shipped** 2026-06-17) — the engine semantics are
frozen: kernel rules, NAF at the closure/world boundary (S1.21.8),
branching, no-good learning, the set-indexed lattice engine.
**Blocks:** [M1b](../m1b_gui/README.md) — the GUI binds to *the engine
that ships*; landing ein.rs first means M1b binds once, and after M1b's
2026-08-18 stack decision it binds by linking these crates into a Tauri
backend rather than talking to a process.
[M2](../m2_nl_to_ir/README.md)'s NL frontend has **no settled boundary**:
"it stays CPython for llama.cpp" was this document's claim until 2026-08-21,
when P1a.9's census found llama.cpp is reached over HTTP and CPython is not
forced. The binding is deferred
([Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding))
and the frontend's language is M2's own decision.
---

> **Instruments (M1a [S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md)).** This document names `profile_solve.py`, `ein-conformance` and `ein-oracle`. They are gone — deleted with the second engine at S1a.10.3–S1a.10.5 — so the numbers here are a **record**, not something you can re-run. What answers each one's question now is the census in [`utils/README.md`](../../utils/README.md#the-census).

## The decision

The placeholder deferred "**Boundary A** (full port) vs **Boundary B**
(hot-loop port behind PyO3)". **Resolved 2026-08-17: Boundary A.** ein.rs
re-implements the whole stack — IR parser, KB, engine, renderers, CLI —
as a standalone binary. PyO3 becomes an *output* of the port (P1a.9), not
its boundary — and P1a.9 then **deferred the output too**, on 2026-08-21, no
consumer having asked for it.

Two invariants govern every stage, and they pull in opposite directions
on purpose:

> **I1 — Outside, nothing changes.** ein.rs is a drop-in replacement for
> `ein`: same surface language, same CLI, same stdout bytes, same exit
> codes, same DOT, same markdown trace, same verdicts, same counters,
> same error messages. Any observable difference is a bug in ein.rs, not
> a design liberty. `ein.py/` stays in the repo permanently as the
> **oracle**.
>
> **I2 — Inside, everything is on the table.** Atoms and facts become
> integers, tuples become flat interned rows, the fork becomes a
> zero-copy layer, the matcher becomes a register machine, the search
> layer runs on many cores, and a loaded KB can be stored and mapped
> back from a binary file. None of that is allowed to leak through I1.

I1 is what makes I2 safe. A rewrite with a byte-exact oracle is a
*measurable* rewrite: every optimisation is either parity-preserving or
rejected, and "did I break the semantics?" is answered by a harness, not
by reading. That is why P1a.0 (the conformance harness) comes before a
single line of engine code.

### Why a port at all (recap)

The placeholder's three reasons stand, and the numbers below sharpen the
second:

1. **Distribution.** M1b (GUI) and M2 (NL) ship to users; PyPy adds a
   second interpreter to install, ein.rs ships one binary.
2. **The hot loop is data-model-bound, not interpreter-bound.** See
   § Baseline: `_bind_arg` allocates a fresh `dict` per bound variable
   and compares interned-by-accident Python strings. That cost is
   structural, and PyPy only shaves a constant off it.
3. **Concurrency.** The lattice layer is embarrassingly parallel and the
   GIL forbids it. P1a.7 turns 101 independent enterings into 101
   independent tasks.

A fourth reason arrived with the F9/F11 ledgers: **the remaining named
levers are ones Python cannot hold.** [F11](../followups/f11_deductive_layer_perf.md)
parks RETE beta-memories precisely because "a memory that must be copied
per fork can lose more than it saves" — a problem that dissolves the
moment a fork is an `Arc` + a delta instead of a dict copy (see
[design/03](design/03_data_model.md)). F11 names the Rust port as its own
most likely promotion trigger.

---

## Baseline — what ein.rs has to beat

Measured 2026-08-17 on the dev machine, `examples/` unmodified,
`master` @ `601f002`. Read the *ratios*; the absolutes are
machine-specific.

| workload | CPython 3.14 | PyPy 3.11 |
|---|---:|---:|
| `solve zebra2.ein` (default, `stop_after=1`), end-to-end | 1.87 s | — |
| `solve zebra2.ein -e` (exhaustive), end-to-end | 5.69 s | 4.07 s |
| `solve zebra.ein -e` (exhaustive), end-to-end | — | 8.15 s |
| — of which: parse | 0.20 s | 0.27 s |
| — of which: kb load (imports + macro expansion + index build) | 0.43 s | 0.37 s |
| — of which: root saturation | 0.09 s | 0.32 s |
| — of which: hypothesis search | 4.96 s | 7.18 s |

Attribution (CPython + cProfile, `utils/profile_solve.py --exhaustive`,
zebra2, 20.4 s profiled / 74 M calls):

| site | self | cumulative | calls |
|---|---:|---:|---:|
| `match._bind_arg` | 20 % | 6.4 s | 6.0 M |
| `match._bind_args` | 18 % | 10.7 s | 4.6 M |
| `builtins.isinstance` | 14 % | — | 31.9 M |
| `match._run_steps` | 6 % | 12.3 s | 1.0 M |
| `saturator._binding_key` (+ genexpr) | 7 % | 2.7 s | 445 k |
| `engine._hashable` | 4 % | 1.2 s | 2.5 M |
| **`saturator._admit_from_boundary` → `World.first_failing`** | — | **14.7 s (72 %)** | 3.2 k rounds / 33 k guard queries |
| `fork` / index copy | 0.01 % | 0.003 s | 206 |

Three readings drive the design:

- **The matcher is the machine.** 46 % of self time is the match/bind
  subsystem, most of it unification — `isinstance` dispatch on IR node
  types plus a `{**bindings, name: arg}` dict copy *per bound variable*.
  [design/05](design/05_matcher.md) replaces both: slot-numbered
  registers with a backtrack trail, and a 4-byte `Value` compared by
  integer equality.
- **NAF costs more than the closure.** `_admit_from_boundary` dominates
  the exhaustive run: the same guard sub-plans are re-queried at every
  quiescence, throttled only by the `_watch_stamp` invalidation check.
  This is where an incremental negative index pays
  ([design/06](design/06_saturation.md) § Boundary).
- **The Python fork is already free** (0.003 s / 206 calls) — so the COW
  work is *not* about beating the current fork, it is about making
  hundreds of thousands of forks affordable so P1a.7 can run them in
  parallel and P1a.6 can afford beta-memories.

**Targets** (all at `--jobs 1`, so they measure the port and not the
cores): ≥ 20× on `solve zebra2 -e` end-to-end vs PyPy, ≥ 50× on parse +
load, and the `ein.py` (deleted at S1a.10.5) acceptance gate under 5 s.

> **Re-measured again at [S1a.6.1](p1a.6_performance/s1a.6.1_profile_baseline.md)
> (2026-08-18), and this whole section is superseded by
> [p1a.6_performance/baseline.md](p1a.6_performance/baseline.md).** Two of the
> PyPy figures moved *up* — `zebra2 -e` 4.07 → **4.94 s**, `zebra -e` 8.15 →
> **8.79 s** as processes — and one, the 0.78 s for parse + load, cannot be
> derived from its own components on either interpreter. The attribution above
> is ein.py's and still describes ein.py; **it does not describe ein.rs**,
> whose exhaustive `zebra2` is 59.7 % saturation / 29.0 % matcher where the
> table above says 46 % matcher and a 72 % boundary — and whose `zebra -e` is
> 66.9 % matcher, so the two puzzles no longer agree either. Read the old numbers as
> the reason the port was started, not as a description of what it became.
>
> **Re-measured at [P1a.0](p1a.0_conformance_harness/README.md), as this
> section asked: the acceptance gate is 43.7 s under PyPy 3.11, not the
> ~91 s recorded at S1.21.8** — 21 tests, `./run_tests.sh
> --acceptance-only`, 2026-08-17. Some of the gap is the machine and some
> is S1.9.E23's fail-fast fork saturation, which landed after that
> recording and cut ~64 % of dead-fork saturation time; the split is not
> worth chasing. What matters is that the "under 5 s" target is **~9×**,
> not the ~18× the stale number implied. The target stands; the claim
> about it does not.

These are targets, not promises; each phase records what it actually got
in [design/README](design/README.md) § Measured.

---

## Shared assets — one stdlib, one example corpus

Both implementations read the **same** `.ein` files. That is a hard
requirement, not a convenience: a forked stdlib would make every parity
result meaningless.

- `ein.py/src/ein/stdlib/*.ein` moves to repo-root **`stdlib/`**; the
  Python package keeps a build-time *copy* so wheels are unaffected, and
  a CI hash-manifest check fails if the copy drifts. ein.rs embeds the
  same tree in the binary (`include_dir!`).
- Resolution order is identical in both: `$EIN_STDLIB` → repo-root
  `stdlib/` when running from a checkout → the packaged/embedded copy.
- `examples/` stays where it is; both test suites enumerate it from the
  repo root, and the conformance corpus is derived from
  [`examples/README.md`](../../examples/README.md)'s catalog.

Full contract: [design/11](design/11_shared_assets.md).

---

## Phases

| phase | title | stages | est. | gate |
|---|---|---|---|---|
| [P1a.0](p1a.0_conformance_harness/README.md) ✅ | Conformance harness + shared assets | 4 | 2 w | **shipped 2026-08-17** — whole corpus 556 cells, 0 diff at T3; same across hash seeds |
| [P1a.1](p1a.1_ir_frontend/README.md) ✅ | IR frontend — lex, parse, AST, dump, macros, imports | 3 | 2 w | **shipped 2026-08-18** — dump / resolve / minimise / expand byte-identical on the corpus; 2.2 M fuzzer mutations, 0 diff; parse 1 003× |
| [P1a.2](p1a.2_kb_core/README.md) ✅ | KB core — interner, values, store, indexes, loader, provenance | 4 | 2.5 w | **shipped 2026-08-18** — 95 corpus files at KB-shape parity, every load error byte-identical; `fork` O(1) under a counting allocator; load 607×, RSS 15× |
| [P1a.3](p1a.3_deductive_core/README.md) ✅ | Deductive core — compile, match, saturate, world, contradiction | 4 | 3.5 w | **shipped 2026-08-18** — T2 on 64 files / 23 848 events, 0 diff; zebra 502 and zebra2 378 facts; `saturate_root` 31×, `match_hot` 55× |
| [P1a.4](p1a.4_search_layer/README.md) ✅ | Search layer — hypgen, lookahead, apriori, nogoods, lattice solve | 6 | 4 w | **shipped 2026-08-18** — 65 files at verdict + counter parity in three regimes; the three acceptance fixtures in 0.87 s; `solve zebra2 -e` 26× |
| [P1a.5](p1a.5_presentation/README.md) ✅ | Presentation — trace, DOT, dumps, CLI | 4 | 3 w | **shipped 2026-08-18** — T3 corpus-wide, 472/473 cells byte-identical, the one exception being D2; help *content* parity by structural diff (Q-M1a.13); **I1 discharged** |
| [P1a.6](p1a.6_performance/README.md) ✅ | Performance — the optimisation programme | 12 | 3.5 w | **shipped 2026-08-20** — **all four targets met 2026-08-19** (S1a.6.8, S1a.6.9) and held with **88 % of headroom** after S1a.6.12 — `solve zebra -e` **585.8 → 47.5 ms** across the phase, `zebra2 -e` → 28.9 ms, both ~**165× PyPy**. Parity is no longer *byte*-unbroken and that is a decision: a fork resuming root's saturation narrates a quarter as much ([D3](divergences.md#d3--a-fork-resumes-roots-saturation-einpy-re-derives-it), [Q-M1a.18](open_questions.md#q-m1a18--may-a-fork-stop-re-narrating-the-roots-fixpoint)), so S1a.6.10 moved the contract to *what a fork derives* — **T3 and T2 clean apart from [D2](divergences.md), whose *two* shapes are the only differing cells** — and S1a.6.11 replaced the elided bytes with twelve ein.rs goldens. The phase closes on its two instruments: a lever matrix that drives **both** engines and carries a **control** row pricing each column (1.2× PyPy, 1.0× ein.rs), and a **differential fuzzer** that found **four parity bugs in its first twenty minutes** — all fixed — plus D2's second shape and two D3 reaches recorded in the ledger. Five of six acceptance items met; the sixth is ≥ 24 h of fuzzing, which is calendar time |
| [P1a.7](p1a.7_parallelism/README.md) ◑ | Parallelism — deterministic multi-core search + match | 6 | 2.5 w | **resumed 2026-08-22** after a two-day pause taken at the user's direction while P1a.8–P1a.10 ran. [S1a.7.0](p1a.7_parallelism/s1a.7.0_speculation_audit.md) shipped 2026-08-20 and measured the phase's central risk before building any of it: **1 078 704 enterings speculated against layer-start root**, the control clean on all 1 078 154 case-1 ones, and the re-validation rate **0.1 % corpus-wide but 36–50 % on the zebra family** — where **35** speculations return `alive` for an entering the sequential engine kills. Every one is in layer 1, which is the only layer that writes to root mid-layer, so [design/08](design/08_parallelism.md) §2's "case 1 is the whole of layer 1" is **inverted** and the layers where 98–100 % of a real search lives need no validator at all. **The resumption's first act was the restatement the pause made necessary**: four of the five remaining stages wrote their acceptance in T0–T3, and P1a.10 retired the harness those name. The successor is not a weaker gate but a sharper one — [`ein-parity`](../../ein.rs/crates/ein-parity/src/lib.rs)'s **cut**, which holds the verdict, the model, the unsat core and *every search counter* exactly and admits only narration, applied through the `corpus_ops` sweep that [S1a.10.1](p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md) already runs over the manifest's 128 files under a permuted id space (51 of 3 160 permuted pairs move, all narration, 0 answers). "T3-identical" becomes "identical except where a permuted id space already moves, and no wider", which a reviewer can check rather than only read; and it is ~2 000× cheaper than the two-process sweep, which may cost T1a.7.5.3 its nightly cadence in favour of the gate. Then **[S1a.7.1](p1a.7_parallelism/s1a.7.1_sync_shared_state.md) measured its own premise and lost two tasks to the answer** ([shared_state.md](p1a.7_parallelism/shared_state.md)): design/08 §6 specifies three *write* strategies and nobody had taken the write rate. A search assigns **41 to 417 fact ids** — `features/01 -e` is **41 across 384 167 enterings** — against **5.8–26 M** borrow-returning reads, so the lock-free segmented vec is built for an append that fires tens of times per *solve*; and counted **per entering**, which is what a worker runs, four of the six workloads have **zero** and the worst has 7 of 111, every one of them in the head of its layer. So T1a.7.1.2 is decided rather than benched: workers hold `&FactStore`, `intern` stays `&mut` and therefore stays on the committing thread, the type system is the enforcement, there is no protocol for `loom` to model, **fact-id assignment stays deterministic under `--jobs N`** — and the `boxcar` dependency is rejected on the record rather than reserved. What the measurement found instead is the shared structure design/08 §6 has **no row for**: the **provenance arena**, written by 100 % of enterings, 2 135 093 records and **205 MB** on `features/01 -e`, with the same borrow-returning read — and it is where the phase's "memory scales with jobs" risk actually lives, since that file peaks at 724 MB at `--jobs 1` and nothing reclaims a dead fork's records until the run ends. New task **T1a.7.1.7** — **decided the same day**, and by a route the first guess would have missed: "promote only the forks that live" looked dead, since **92.3–100 % of enterings are alive**, until the question was asked the other way round. An alive entering's fork is *dropped* after the `complete()` probe and the dumper hook; only `record_node`'s snapshot of a **solution** retains anything. So `features/01 -e` creates 2 135 093 records and, when the solve ends, **nothing references one of them** — and the arena is read **15** times against those 2.1 M pushes, so a second one costs nothing. The arena is therefore **per-worker**, with promotion only on the solution path (zero enterings on four of the six workloads), and the claim is asserted in both directions: `ProvArena::retire` + a panic in `get` arms **every debug build**, so the whole gate is the experiment, and `ein-infer/tests/provenance.rs` adds the stronger holding-side check a reclamation needs (**5 328 live justifications over 90 files, none retired**) — because an id that is stored and never read trips no read-side assertion and would still be corrupted by reuse. Arming it found exactly one reader and the finding was the distinction rather than a bug: `ein-einb`'s writer walks the arena **end to end**, which is a *scan* and not a reference, now named `ProvArena::scan`. **And it was then built, the same day.** Not the one-line truncate it looks like, because `handle_dead` pushes root's records *after* the fork's so the retired range is never the tail: what shipped is a **fork region** on the arena that `push` routes into, with three verbs rather than two — `close_fork` stops the routing one step before `discard_fork` frees, because the dumper still has to render a dead fork's justifications after root has written its no-good. Reuse is caught in **release too**: the region's base is monotone, so a stale id falls below it and `get` panics rather than addressing the wrong record, which is strictly stronger than the debug-only bitset it replaces. `Kb::promote_provenance` copies out what a solution cites, in the fork's own push order so no id depends on a hash walk, and clones no layer that does not cite one. It pays twice: the last shared mutable structure leaves a worker's path — **every structure design/08 §6 named is now `&`-shared or per-worker, so there is no protocol for `loom` at all** — and the memory comes back *sequentially*, which is where "memory scales with jobs" turned out to live: `features/01 -e` goes from **684–708 MB peak RSS to 85–91 MB** at `--jobs 1` (S1a.6.4 read the same cell at 724 MB; the A/B re-took both sides from one commit apart) and from 1.97 s to **1.68 s**, `sq-bwd/houses -e` from 93 to 17 MB, and five of the six workloads end with an arena under half a megabyte. The read path is **not slower** on the two read-heaviest workloads either (0.92 → 0.90 s, 0.28 → 0.25 s), which settles that acceptance item early in the shipping build rather than in a bench. The gate is green in both profiles at **587 tests with no re-bless**, and `provenance.rs` is re-pointed from "is this id retired" to "is this id a fork's" — checkable in every build — and extended to the recorded solutions, which is the step promotion could get wrong: **7 037 live justifications over 90 files and 65 solution nodes, none of them a fork's, 6 773 records promoted**. And the interner needs **no lock at all**: between the end of root saturation and the end of a solve, **four** distinct names arrived, on 24 of the 90 corpus files that solve — three the engine's own (now `ein_core::terms::ENGINE`, interned with the kernel vocabulary) and one a rule's *argument* constant (`Ann`, in an `hrule`'s `:assert`, which no fact mentions and which the loader therefore never saw; now interned at load) — with the integer pool never growing. So `&Interner` is `Sync` already, `text` keeps returning a borrow, and not one read site changes. `ein-infer/tests/interning.rs` holds it at 0 of 90. The plan memo had been done since S1a.6.8. Also shipped: `bench_env.sh --cores P:8|PT:8|E:8`, without which no `--jobs N` number can say which of three machines "8 cores" meant |
| [P1a.8](p1a.8_binary_container/README.md) ✅ | Binary KB container — `.einb`, mmap, solution store | 1 | 0.5 w | **shipped 2026-08-21** — one stage, one crate. `ein-einb` is the **eighth** workspace member and the only one that is not `#![forbid(unsafe_code)]`: design/12 §2 allows `unsafe` in exactly one audited module and `forbid` cannot be lifted per-module, so the crate boundary is what makes "exactly one" a fact. A saturated `zebra2` is **57 688 bytes** and opens cold in **0.614 ms**, and `ein solve x.einb` is byte-identical to `ein solve x.ein` across four puzzles and five diagnostic flags with **two** lines normalised — the path `solve` echoes and `--stats`'s wall clock. Two design questions were answered by measurement rather than by argument: `PROGRAM` is **canonical text** and not the AST arenas, because the arenas for a resolved `zebra2` are past 60 KB against a 64 KB budget while `dump_canonical` of the same forms is 11 KB; and there is **no `INDEXES` section**, because `rebuild_indexes` *is* the projection that defines them. That second decision found the stage's one real bug: `rules_by_relation` is **not** a projection — it is taken once at the end of `load` and shared by reference, so rebuilding it from a saturated fact set produces a larger map than the KB ever had (`Kb::rebuild_indexes_from`). The bit-flip sweep found the other: the digest covers everything *after* the header, so the header's reserved words are now required to be zero. `SOLUTIONS` ships with a library API and **no CLI producer** — [F9](../followups/f9_e_catalog.md)'s measurement hazard handled structurally, with a test that keeps the solve path unable to read one |
| [P1a.9](p1a.9_release/README.md) | **Release** — the slow corpus, packaging, docs | 3 | 1.2 w | **Re-topiced 2026-08-21 from "Bindings + release".** It was four stages and two of them were PyO3; the census killed both. [M1b](../m1b_gui/README.md) links the crates, [M1c](../m1c_external_validation/README.md)'s runner **must** shell out to be a fair measurement ("`cargo build` never needs Z3"), `utils/` names its binary on purpose — and M2, the sole remaining consumer, does not force CPython: its llama.cpp is a `llama-server` container reached over HTTP, and the acva pattern it mirrors has a **C++** client. Worse for the binding, the one thing that wanted it — a validator that needs *why* a load failed, as data — is exactly what a Rust frontend gets for free by linking `ein-ir`. Deferred with three named trip-wires ([Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)), the likeliest being [M2b](../m2b_presentation/README.md)'s artifact reviewers; `docs/api/`'s 1 051 lines are kept intact as the specification a trip-wire would restore. In their place, the stage the phase would have shipped without: **[S1a.9.0](p1a.9_release/s1a.9.0_slow_corpus.md)**, because the corpus's 17 `slow = true` entries are priced against CPython — the exclusion notes say so in words — and re-measuring found `zebra2` at **11 ms** still flagged slow, `features/04_open` at **136 s** still unfinishable on an engine 165× PyPy, and **ten entries where `solve` costs exactly what `solve -e` costs** because all ten return `Contradiction` / `exhausted=False` at `layers == max_set_size`: the search is running out of commitment depth, not proving unsat. An `-m 2…7` sweep confirms it (layers tracks `-m`, enterings 120 → 16 383, verdict never changes) and the verdict-word question goes to [M1d](../m1d_satisfiability/README.md) |
| [P1a.10](p1a.10_single_implementation/README.md) ✅ | One implementation — port the suite, retire ein.py, the harness and the submodules | 6 | 3 w | `cargo test --workspace` is the whole gate, and coverage did not drop. **[S1a.10.1](p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md) shipped 2026-08-20** — run ahead of P1a.8/P1a.9 because the dependency is the *deletion*'s, and it found that the gate is already half differential: **42 of `cargo test --workspace`'s 91 integration tests start a Python process**, and skip invisibly when one will not start. The [ledger](p1a.10_single_implementation/oracle_ledger.md) banks the rest in three instruments — 4 228 renderings as digests, 13 counter identities, and a determinism sweep that permutes the **id space** instead of a hash seed and prices it at **0 answers and 66 renderings**, all of them [D3](divergences.md)'s. **[S1a.10.2](p1a.10_single_implementation/s1a.10.2_port_the_suite.md) shipped the same day**: the Python suite's 1 538 tests reduce to **275 behaviours** in fifteen new Rust files ([dispositions](p1a.10_single_implementation/suite_dispositions.md), per file, with the 96 dying subjects named), and all 42 differential tests are un-differential. `cargo test --workspace` is **566 tests in 1 m 07 s** where it was 312 in 9 m 13 s, and `PATH=<a python3 that exits 127>` now leaves all 566 passing where the same experiment found 41 silent skips. **[S1a.10.3](p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md) shipped 2026-08-21**: the harness itself — `ein-conformance`, `ein-oracle`, T0–T3, 2 164 lines — retires, and the corpus it read becomes [`corpus/`](../../corpus/README.md) with a runner that is a **sweep** rather than a diff (`ein-cli/tests/corpus_cli.rs`, 542 cells as processes in 2.5 s, a 660-line exit table, and a per-cell timeout so a non-terminating program fails the gate instead of hanging it). The exit table is banked rather than ruled because the corpus does not obey a group rule: `render rules` never loads the KB, so **10 of 30 load-negatives exit 0**. **[S1a.10.4](p1a.10_single_implementation/s1a.10.4_utils.md) shipped 2026-08-21**: `utils/` is **17 scripts** rather than 28, all of them driving ein.rs, and every CPython/PyPy number in `baseline.md` and `features.md` is now labelled a frozen constant because its instrument is gone. The fuzzer keeps its generator and loses its differ — five properties one engine can check, the strongest being `id_order_invariance` pointed at generated input through a new `EIN_ID_FILES` seam — and **found three things in twenty minutes**: a `debug_assert!` an ordinary `(hrule …)` reading `not` trips, an **unsat core** whose contents depend on interning order, and the same for the goal-binding row the solve table prints, which re-derived a finding filed in August against [D3](divergences.md) and showed that **D3 perturbs that row rather than being why it is perturbable** — it moves inside one engine too. **[S1a.10.5](p1a.10_single_implementation/s1a.10.5_removal.md) shipped 2026-08-21**: `ein.py/` is gone, 183 files, tagged `two-implementations` at the parent. The Lark grammar became EBNF *first* (`01_grammar.md` §3, the user's own precondition promoted to T1a.10.5.0), and T1a.10.5.1's acceptance was **amended on evidence** — `nlp/` and `smt/` have named dependents in M1c and M2, so the two *submodules* are deinitialised and the directories stay. **[S1a.10.6](p1a.10_single_implementation/s1a.10.6_docs.md) shipped 2026-08-21 and closes the phase**: the removal's 224 dangling links resolved into ~150 module pointers with a 1:1 counterpart, ~60 `.py`-named link texts, **46 symbols ein.rs does not have** and **~20 claims that only made sense with two engines** — and that last 9 % is what the stage was for. Its output is [`docs/kernel/defined_behaviour.md`](../../docs/kernel/defined_behaviour.md), **thirteen behaviours whose only statement was a Python source file**, now normative — and enumerating them found two that are *bugs* rather than quirks: the binding key that drops non-string activator args ([Q-M1a.8](open_questions.md#q-m1a8--binding_key-drops-non-string-activator-args) — a puzzle with integer rule parameters can lose a firing, silently) and the six Python exception classes the CLI prints, which are now a name with no referent. `python_impl.md` was **renamed and re-aimed** rather than deleted, because `docs/kernel/README.md`'s dev path is the only orientation into the code and ein.rs has no README; 24 plan documents gained a one-line instrument marker; `AGENTS.md` lost its `ein.py/` bullet. **Phase acceptance: all met** — `cargo test --workspace` is 542 tests over 58 targets, 0 failures, no Python process in any of them |

**54 stages** (S1a.6.8 added by S1a.6.1's profile, S1a.6.5 shortened by it,
S1a.6.12 written at S1a.6.5 against the profile that had named it since
S1a.6.3, and S1a.7.0 added at P1a.7's start by the same reflex that added
S1a.6.1 — measure the premise before spending four days on it; **P1a.10–12
added 2026-08-20** at the user's direction, 16 stages and 8.5 weeks, of which
**P1a.11 and P1a.12 left the next evening** for M1c and M1d, 10 stages and
5.5 weeks; and **P1a.9 net −1 on 2026-08-21** — two binding stages cut, one
added by the same reflex that produced S1a.6.1 and S1a.7.0, which is now
three for three), 156 days of stage estimates ≈ 31 weeks. The count is a correction
as well as a subtraction: the header read 45 and the table summed to 64, and
after the P1a.10–12 batch neither was right. The **parity gate**
(end of P1a.5) is at ~week 17; everything after it is speed, scale and
distribution on an engine that is already a drop-in replacement.

> **P1a.11 and P1a.12 became [M1c](../m1c_external_validation/README.md) and
> [M1d](../m1d_satisfiability/README.md) on 2026-08-21** — added to this
> milestone on the 20th, re-homed the next evening at the user's direction,
> with their stage files, estimates and dependencies unchanged
> ([P1c.1](../m1c_external_validation/p1c.1_stdlib_conformance/README.md),
> [P1d.1](../m1d_satisfiability/p1d.1_exhaustive_search/README.md)). Neither
> was the port. P1a.11 adds *language surface* — `:expect`, and `query`
> becoming plural — and P1a.12 changes *when the search may stop*; both stood
> in § Non-goals as named exceptions to the two rules that define this
> milestone, and moving them out is a cleaner resolution than narrowing a rule
> twice. What M1a keeps is exactly what I1 and I2 describe: an engine that
> behaves like ein.py and is written differently inside. **Both exceptions are
> gone from § Non-goals with them.**

> **P1a.8 was "Server mode" until 2026-08-18** — 8 stages, 3 weeks:
> daemon, sessions, JSON-RPC, streaming, a solution cache and `ein <cmd>
> --server`. Dropped: nothing downstream needs a resident process.
> [M1b](../m1b_gui/README.md) settled on Tauri, whose backend *is* a Rust
> process linking these crates directly; [M2](../m2_nl_to_ir/README.md)
> crosses into CPython through PyO3 (P1a.9) — **that half is superseded**:
> the binding was deferred 2026-08-21 and M2's boundary is open, which
> strengthens rather than weakens the argument, since the alternative it
> gained is *linking the crates* and not a socket; the CLI is the only other
> consumer. The one deliverable that was never about the daemon — the
> `.einb` container — stays as a single stage. The seven server stages and
> `design/09` are in git history.

Ordering rationale: **parity first, speed second, scale third.** P1a.0–5
land a slower-than-Python but byte-identical engine; only then does
P1a.6 start trading representation for time, with the harness watching.
Doing it the other way round means every regression is ambiguous.

---

## Non-goals

- **Re-deriving M1's semantics.** Every invariant M1 established
  (S1.5a.1 NAF re-eval, S1.5a.1a determinism, S1.7.23 no kernel type
  system, S1.21.8 closure/boundary, P1.21 R2 root stability) is a *port
  target*, not a redesign target. Where a Python behaviour looks wrong,
  the fix belongs in ein.py first — then both ports move together.
- **A "Rusty" reinterpretation of the IR.** No new syntax, no new
  keywords, no relaxed grammar. `grammar.lark` stays the spec of record
  (M2's GBNF lift reads it); the Rust parser is checked *against* it.
  **The 2026-08-20 narrowing is withdrawn 2026-08-21**: it was written so that
  P1a.11 could add one form once there was only one parser to add it to, and
  P1a.11 is now [P1c.1](../m1c_external_validation/p1c.1_stdlib_conformance/README.md).
  For M1a the rule is strict again. The half that was never about the port
  travels with the phase: `grammar.lark` stays the spec of record, and a new
  form is a **cross-milestone edit** because M2's GBNF lift reads it.
- ~~**Deleting ein.py.**~~ **Reversed 2026-08-20 — it is now
  [P1a.10](p1a.10_single_implementation/README.md).** It read: "It is the
  oracle and the reference for M2 experiments. It stays, and stays green."
  The case for the oracle was never that a second implementation is valuable
  in itself — it was that a rewrite with a byte-exact oracle is a *measurable*
  rewrite, and that argument expired when the byte gate closed at the end of
  P1a.5. P1a.6 already lives past it: [D3](divergences.md) is a deliberate
  divergence and [S1a.6.11](p1a.6_performance/s1a.6.11_fixture_goldens.md)
  replaced the elided bytes with ein.rs's own goldens. What the reversal costs
  is falsifiability, permanently, and
  [S1a.10.1](p1a.10_single_implementation/s1a.10.1_bank_the_oracle.md) is the
  gate that prices it: nothing is deleted until every claim the harness
  carries has a checked-in owner. **I1's "ein.py stays permanently as the
  oracle" is amended the same way and for the same reason.**
- ~~**Dropping PyPy support.**~~ **Reversed 2026-08-20 with the above** —
  there is no Python engine left to run under PyPy. Python was to stay a
  supported *consumer* through [P1a.9](p1a.9_release/README.md)'s PyO3
  module — **and that is deferred too**, 2026-08-21, for want of a consumer
  ([Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).
  What ships is a binary and a set of crates; Python reaches the engine the
  way every other language does, by running it.
- **A resident server.** Dropped 2026-08-18 (see the phase table).
  ein.rs ships a **library and a CLI**; an embedder that wants
  load-once/ask-many holds the engine in its own process — which is
  exactly what M1b's Tauri backend and M2's PyO3 binding do. No daemon,
  no wire protocol, no solution cache, no `--server` flag.
- **New reasoning features.** Anything that changes what the engine can
  prove belongs in a followup ([F2](../followups/f2_self_modifying_language.md),
  [F4](../followups/f4_cross_cutting.md), [F7](../followups/f7_rule_induction.md))
  or in [M1d](../m1d_satisfiability/README.md), not here. The exception this
  clause carried until 2026-08-21 — P1a.12, "sits on this line and is scoped
  to stay inside it" — left with the phase, and the distinction it drew is
  M1d's to keep: a *sound* stopping criterion proves the same thing sooner and
  costs nothing here; a heuristic that changes the answer ships behind a flag
  with a different verdict word, or not at all.

---

## Design docs

The technical substance lives in [`design/`](design/README.md):

| doc | what it settles |
|---|---|
| [01 — Parity contract](design/01_parity_contract.md) | what "1:1" means, the four tiers, the oracle event protocol, the corpus |
| [02 — Determinism & order](design/02_determinism_and_order.md) | every order-sensitive iteration site in ein.py, and how ein.rs reproduces it |
| [03 — Data model](design/03_data_model.md) | interning, `Value`/`FactId` as integers, row storage, the layered COW KB |
| [04 — IR frontend](design/04_ir_frontend.md) | hand-written lexer/parser, AST arena, dumper, macro expander, import resolver |
| [05 — Matcher](design/05_matcher.md) | plan bytecode, register bindings + trail, indexes, beta-memories, WCOJ |
| [06 — Saturation](design/06_saturation.md) | closure/boundary loop, semi-naive delta, queues, incremental NAF |
| [07 — Search layer](design/07_search_layer.md) | hypgen, lookahead, apriori, nogoods, the monotonic loop |
| [08 — Parallelism](design/08_parallelism.md) | the four parallel levels and how each stays deterministic |
| [10 — Binary format](design/10_binary_format.md) | `.einb` container, mmap, versioning, content addressing |
| [11 — Shared assets](design/11_shared_assets.md) | one stdlib, one example corpus, drift checks |
| [12 — Toolchain & layout](design/12_toolchain_and_layout.md) | crates, dependencies, build, CI, benches |

---

## Open questions

Live questions carry `Q-M1a.<n>` ids in
[`open_questions.md`](open_questions.md). The load-bearing ones at
promotion time: parse-error message parity (Q-M1a.3) and whether
`--jobs > 1` may move counters (Q-M1a.7). The two server questions
(Q-M1a.11 wire protocol, Q-M1a.12 remote access) were **closed moot
2026-08-18** with the server itself. P1a.3 added one the design docs did
not anticipate: [Q-M1a.17](open_questions.md#q-m1a17--win-bs-80--assumed-monotone-guards-dominate),
where Win B's ≥ 80 % target met its own measurement and lost.

P1a.6 answered the one nobody had written down before S1a.6.9 forced it.
**Q-M1a.18** — may a fork stop re-narrating the root's fixpoint? — resolved
**yes, in ein.rs only**, and the principle that moved with it is bigger than
the question: the contract's hard requirement is that the two engines produce
the same *answer*, not the same bytes. T0 and T1 stay exact and are compared
more carefully than before; narration parity was a means that had served its
purpose, and ein.rs's regression coverage moved to checked-in goldens.

**Q-M1a.19, Q-M1a.20 and Q-M1a.21 left 2026-08-21** with their phases —
they are [Q-M1c.1](../m1c_external_validation/open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects),
[Q-M1c.2](../m1c_external_validation/open_questions.md#q-m1c2--what-may-an-expectation-say)
and [Q-M1d.1](../m1d_satisfiability/open_questions.md#q-m1d1--may-the-search-stop-before-the-lattice-is-exhausted)
now. The M1a ids stay reserved and their entries redirect: a sticky id that
disappears is worse than one that points somewhere.

P1a.4 closed the two it was blocking on. **Q-M1a.4** — `sorted()` over
mixed-type fact args — became the ledger's
[D2](divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)
once `layer_1` was reachable: exactly one corpus file diverges, exactly
the predicted one, and the parity sweep *asserts* the divergence rather
than tolerating it. **Q-M1a.5** — CPython's `random.shuffle` — was
resolved by porting MT19937, checked by table and then on every corpus
entry through a seeded `solve` regime.

## Cross-links

- [`docs/kernel/`](../../docs/kernel/README.md) — the specification
  ein.rs implements. `inference/architecture_and_algorithms.md` §O1–O9 is
  the operation-by-operation map every design doc here refers back to.
- [`docs/api/ein.md`](../../docs/api/ein.md) — the Python embedding
  contract, now **a record held in reserve**: the surface that was to keep it
  is deferred
  ([Q-M1a.23](open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)).
- [F11 — deductive-layer perf](../followups/f11_deductive_layer_perf.md)
  — D1 (beta-memories) and D2 (WCOJ); this milestone is their promotion
  trigger. Absorbed by [P1a.6](p1a.6_performance/README.md).
- [F10 — M1 refactor-debt tail](../followups/f10_m1_refactor_tail/README.md)
  — closed 2026-08-17; nothing left blocking the port.
- [F9 — E-catalog](../followups/f9_e_catalog.md) — the *rejected*
  search-layer optimisations. Read before proposing one here: most were
  measured inert against a complete cardinality-BFS, and a Rust rewrite
  does not change that arithmetic.
- [M1b GUI](../m1b_gui/README.md) · [M2 NL → IR](../m2_nl_to_ir/README.md)
  — the downstream consumers.
- [M1c — External validation](../m1c_external_validation/README.md) ·
  [M1d — From saturation to satisfiability](../m1d_satisfiability/README.md)
  — created 2026-08-21 out of this milestone's last two phases plus the F14
  note; both depend on M1a and neither is part of the port.
