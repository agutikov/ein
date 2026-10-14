# 12 — Toolchain, workspace layout, and CI

**Settles:** where the Rust code lives, how it splits into crates, what
it is allowed to depend on, and what the gates are.
**Phase:** [P1a.0](../README.md#p1a0--corpus-and-shared-assets), stage
[S1a.0.4](../README.md#s1a04--workspace-skeleton-and-ci).

---

## 1. Repo layout

`ein.rs/` sits at the repo root, beside the assets it reads:

```
ein/
├── stdlib/                     ← shared, single source of truth (11)
├── examples/                   ← shared corpus
├── conformance/                ← corpus manifest + expected messages
├── ein.rs/
│   ├── Cargo.toml              (workspace)
│   ├── rust-toolchain.toml     (pinned stable + components)
│   ├── deny.toml               (cargo-deny: licences, advisories)
│   └── crates/
│       ├── ein-core/           values, interner, KB, indexes, provenance, repr
│       ├── ein-ir/             lexer, parser, AST, dumper, macros, imports, stdlib embed
│       ├── ein-infer/          compile, match, saturate, world, search, verdict, explain
│       ├── ein-einb/           the `.einb` container            [P1a.8]
│       ├── ein-render/         DOT renderers, markdown trace, solution table
│       ├── ein-cli/            the `ein` binary
│       ├── ein-python/             PyO3 bindings (maturin)     [reserved, deferred]
│       └── ein-narration/      what is narration rather than content (dev-only) [S1a.6.10]
└── utils/                      (existing scripts; gains a couple of runners)
```

Why this split:

- **`ein-core` has no I/O and no engine.** It is the data model
  ([03](03_data_model.md)) plus the `repr` value renderer
  ([02](02_determinism_and_order.md) §7). Everything depends on it;
  it depends on nothing.
- **`ein-ir` owns the only filesystem access in the engine** (import
  resolution + the embedded stdlib), so any future policy on what the
  engine may read is one seam rather than an audit.
- **`ein-infer` never formats anything.** All rendering lives in
  `ein-render`, so the printed surface is one crate and can be diffed as a
  unit.
- **`ein-einb` is a crate because of §2's `unsafe` rule, not because of its
  size.** It could have been a module of `ein-infer`, except that every other
  crate is `#![forbid(unsafe_code)]` and `forbid` cannot be lifted
  per-module: a container inside `ein-infer` would have had to unforbid the
  whole engine to get its zero-copy casts. A crate boundary is how "exactly
  one audited module" stays a fact rather than a convention. It sits above
  `ein-infer` (`SOLUTIONS` stores what a solve produced) and below `ein-cli`;
  `ein-render` does not depend on it, so the stack is linear up to
  `ein-infer` and forks there.
- **`ein-narration` is a dev-only workspace member, and it exists for a
  reason a smaller repo would not have.** Added at
  [S1a.6.10](../README.md#s1a610--answers-not-narration), it holds
  one rule as code — *a fork's derivation, and anything keyed on a dying
  fork's stopping point, is narration, not content*. A crate rather than a
  module because several crates' own `tests/` need it, and before it existed
  the same decision was implemented in each of them, each cut made as the
  next test went red. It is `publish = false` and no shipping crate depends
  on it: a renderer that decides what a diff will look at is a renderer with
  an opinion about the contract.

`ein.rs/` compiles to a binary named `ein`.

---

## 2. Dependency policy

The engine's runtime dependency budget is deliberately tiny; every crate
listed has to earn its place, and the *engine* crates (`core`, `ir`,
`infer`) must stay usable in a `no_std`-adjacent, dependency-light build
if that ever matters.

| crate | where | why |
|---|---|---|
| `rustc-hash` (`FxHashMap`) | core, infer | deterministic, fast, non-DoS-resistant hashing — correct choice here, and it removes the `RandomState` iteration-order hazard ([02](02_determinism_and_order.md) §9) |
| `smallvec` | core, infer | premises, bindings, commitments — all small and hot |
| `bitvec` *or* a hand-rolled `BitSet` | core | presence / negated / alive sets. Hand-rolled is ~80 lines; prefer it |
| `md-5` | render | `hashed_id` — a stable digest, never a salted hash ([02](02_determinism_and_order.md) §8) |
| `sha1` | render | `palette::hash_color`'s index — the same argument one digest over ([S1a.5.1](../README.md#s1a51--dot-renderers)) |
| `blake3` | ein-einb | the container's header digest — content addressing (design/10 §2) |
| `include_dir` | ir | embedded stdlib |
| `memchr` | ir | lexer scanning |
| `rayon` | infer | [P1a.7](../README.md#p1a7--parallelism) only, behind a `parallel` feature — **taken up 2026-08-22** at [T1a.7.2.1](../README.md#s1a72--level-1-parallel-enterings), and the feature is default-on so `--jobs N` works in the shipped binary. What it buys over `std::thread::scope` is not the API: a fanned-out layer runs in **bounded batches**, so the results in flight cannot grow with the layer, and that makes the cost of a *barrier* the thing to watch — spawning `jobs` threads per batch is ~96 000 spawns on `features/01 -e` and a **3× slowdown** at `--jobs 2`, measured. The threads have to live between batches. The pool is built once per solve and only when `jobs > 1`, so a default run creates no thread at all |
| `boxcar` (or another lock-free append-only vec) | core | **considered, rejected 2026-08-22.** [design/08 §6](08_parallelism.md#6-what-must-be-sync-and-how) sketched the fact store as one, and it would have solved the real problem — `FactStore::args` returns a `&[Value]` into an arena a `Vec` push can move, which `#![forbid(unsafe_code)]` leaves no in-crate way to fix. What killed it is that the problem does not arise: [T1a.7.1.2](../README.md#s1a71--making-the-shared-state-sync) measured **zero** enterings appending a fact id on four of six workloads and 7 of 111 on the worst, so workers hold `&FactStore` and never push. Two dependent loads per read on a 26 M-call path, plus a dependency, for an append that happens at most seven times — the row stays as the record of a decision, not as a reservation |
| `clap` (derive) | cli | 37 options across 8 parsers; help and usage-error *text* are clap's own |
| `serde` + `serde_json` | cli(`--events`) | the event protocol |
| `zstd` | ein-einb | optional section compression. **Not taken up**: P1a.8 shipped the per-section `flags` word that would select it and no compressor, because a saturated `zebra2` is 56 KB uncompressed against a 64 KB budget and a compressed section forfeits the `mmap` the layout exists for. A reader refuses a non-zero `flags` rather than guessing |
| `pyo3` / `maturin` | ein-python | **reserved, not taken up.** P1a.9 was to build the binding and [deferred it 2026-08-21](../README.md#p1a9--release) for want of a consumer — M20 links the crates, M1c's runner must shell out to be a fair measurement, and M2's llama.cpp turned out to be an HTTP server. The crate, this row and the `python` feature stay reserved so the trip-wires in [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding) are cheap to answer |
| `criterion`, `proptest`, `arbitrary` | dev-dependencies | benches and property tests |
| `snmalloc-rs` | cli (binary only), corpus (bench) | the global allocator — [T1a.6.2.7](../README.md#s1a62--memory-layout), added 2026-08-19. glibc `malloc` was **20.0 %** of an exhaustive `zebra2`'s self time; measured against `mimalloc` and `tikv-jemallocator`, this one is the fastest *and* the only fast one that does not cost peak RSS. On the **binary**, never on an engine crate: a library that installs a global allocator makes the choice for everything that links it. Build-time it pulls `cc` + `cmake` |

**Not used:** any parser generator (see [04](04_ir_frontend.md) §1), any
async runtime *anywhere* (with the server dropped there is no consumer
for one; the engine stays synchronous and `Send`),
`lazy_static`/`once_cell` in favour of
`std::sync::OnceLock`, and `unsafe` outside a single audited module for
the `.einb` zero-copy casts (`#![forbid(unsafe_code)]` on every other
crate).

**MSRV**: current stable minus two releases, pinned in
`rust-toolchain.toml` and asserted in CI.

---

## 3. Feature flags

| feature | default | effect |
|---|---|---|
| `parallel` | **on** (`ein-infer`, forwarded by `ein-cli`) | pulls `rayon`, enables `--jobs > 1`. Taken up at T1a.7.2.1. **Forwarded to the binary at S1a.9.3 T1a.9.3.2**, which is when it became droppable at all: `ein-cli` took `ein-infer` with its defaults, so `--no-default-features` on the binary still linked the pool — the feature was real in the engine and fiction in the artefact. The rule the fix settled on is one sentence — **a crate that forwards `jobs` forwards `parallel`**: `ein-render`'s `shape.rs` hands a job count to `SolveOptions`, so it gets its own default-on `parallel` row and `ein-cli` takes it without defaults and forwards both halves; `ein-einb` needs no row, because it never forwards a job count. `ein-render` additionally takes `ein-infer/parallel` as an unconditional **dev**-dependency, since `cargo test --workspace --no-default-features` would otherwise build `jobs_invariance` without a pool and let it compare `--jobs 1` against itself across 20 712 cells; `the_sweep_is_not_vacuous` makes that a failure rather than a green. Off, `--jobs N` parses and is inert — which `ein --version` says by not listing the feature |
| `einb` | on (`ein-cli`) | `.einb` read/write. Off, `ein kb` is not registered and a `.einb` argument is refused by the loader that would have opened it |
| `events` | — | **Not taken up, and measured before it was dropped (S1a.9.3 T1a.9.3.2).** The row reserved a flag that would compile the `--events` emitter out "for a zero-overhead measurement build"; nothing ever added it, and the emitter is unconditional. What it would have removed is one discriminant compare per emit site — `if events.on()`, guarding a closure that is never entered when the sink is `Off`. Priced with the strongest possible version of the feature (`Events::on()` replaced by `false`, so LLVM folds every guard): `solve zebra2 -e` **+3.9 %**, `solve zebra -e` **+1.8 %** — *slower*, at two sample counts, because dead-code elimination moved the layout of a hot function. There is no win here to reserve a flag for ([feature_cost.md §1](../measurements/feature_cost.md#1-the-events-guard--not-worth-a-feature)) |
| `python` | off | PyO3 bindings. Reserved; nothing builds it (see the `pyo3` row) |
| `snmalloc` | **on** (`ein-cli`) | the global allocator (T1a.6.2.7). `--no-default-features` builds against the system allocator, which is what a distro package that would rather not vendor a C++ allocator wants — and costs **+25.2 % of `solve zebra2 -e` and +36.2 % of `solve zebra -e`**, re-taken 2026-08-23 ([feature_cost.md §2](../measurements/feature_cost.md#2---no-default-features--what-a-packager-is-buying)). It read *15.9 % of `solve zebra2 -e`* here until then, which is the 2026-08-19 figure and still true of that engine: nothing about the allocator changed, the engine got 3–8× faster and did not get faster at allocating, so the share grew while the absolute cost fell. The one cell the system allocator wins is `render rules` (−34 %), which is process start-up and snmalloc's arena set-up |
| `fork-delta` | off | the resumed saturator's fixture: compiles the pre-[S1a.6.9](../README.md#s1a69--the-fork-entry-delta-the-resumed-saturator) fresh-fork saturator back in, reachable with `EIN_FORK_DELTA=0` |
| `counters` | off | the work counters ([S1a.6.1](../README.md#s1a61--fresh-profile-and-bench-baseline) T1a.6.1.3), compiled out entirely when off |

**What CI checks about all this, since S1a.9.3.** The release tier builds
`--no-default-features` and runs the suite (607 of 619 tests; the nine that go
are `.einb`'s and the help-surface golden, which is of the default surface on
purpose) — and then asserts the *dependency graph* in both directions, because
"it compiles without them" was true for two years while "it links none of
them" was not. `cargo tree -p ein-cli --no-default-features` must contain no
`snmalloc-rs`, no `ein-einb`, no `blake3` and no `rayon`; the default build
must contain all four. A build that still links a work-stealing pool it cannot
reach compiles perfectly well, which is exactly why compiling is not the
claim.

The paragraph that stood here said benchmarks build `--no-default-features
--features einb` "so no branch on the hot path exists at all". They do not,
they never did, and §3's `events` row now records why they should not: the
branch costs nothing measurable.

---

## 4. CI gates

Three tiers.

**Per-commit (~5 min)**

- `cargo fmt --check`, `cargo clippy -- -D warnings`
- `cargo test --workspace` (unit + property tests)
- `cargo deny check`
- `stdlib-check` ([11](11_shared_assets.md) §3)
- corpus-completeness check

**Nightly (~1 h)**

- the whole corpus × the whole run matrix
- determinism: every corpus entry run twice, byte-diffed
- benches (`criterion`) with regression thresholds
- the fuzzer for a fixed budget, corpus-minimising any find

**Release** — [`.github/workflows/release.yml`](../../../../.github/workflows/release.yml),
written at S1a.9.3 (2026-08-23). Six jobs on a `v*` tag:

- `gate` — the whole per-commit gate plus `EIN_CORPUS_SLOW=1` and
  `EIN_ID_SEEDS=8`, i.e. nightly's depth in one job.
- `jobs-cross-diff` — `EIN_JOBS_SWEEP=2,4,8,16` over
  `ein-render/tests/jobs_invariance.rs`, which is P1a.7's acceptance. It
  replaces a stub that had sat `if: false` since this file was written; the
  phase it was waiting for shipped 2026-08-22.
- `no-default-features` — build, test, and the two dependency-graph
  assertions above.
- `build` — four required legs: `x86_64`/`aarch64` Linux (glibc, on the
  *oldest* runner image, because the glibc a binary links is the floor of the
  systems it runs on), macOS universal2 (two slices, `lipo`), Windows MSVC.
  Each **sweeps the whole corpus on its own platform** — that is the check
  that would otherwise be silent, since line endings, filesystem
  case-sensitivity and locale formatting are properties of the platform and
  not of the target triple — then runs the artefact's own `--version` and a
  solve, and writes a SHA-256 beside it.
- `static-linux` — one musl `--no-default-features` binary, `continue-on-error`
  and **not** in `publish`'s `needs:`. musl has no C++ toolchain on the runner,
  so snmalloc cannot build there; the release notes quote what that costs
  rather than letting a reader pick the static binary blind.
- `publish` — `needs: [gate, jobs-cross-diff, no-default-features, build]`, so
  a red gate cannot ship a binary. It writes one `SHA256SUMS` over the set,
  cross-checks it against each leg's own attestation, generates notes that say
  what is *in* the release (including a missing static binary), and calls `gh
  release create`.

**No wheels** (see the `pyo3` row), and no thread sanitizer: the workspace is
`#![forbid(unsafe_code)]` outside `ein-einb::cast`, the fan-out shares
`&FactStore` and nothing else
([design/08 §6](08_parallelism.md#6-what-must-be-sync-and-how)), and the
question TSan would answer — *do the threads agree?* — is answered directly by
`jobs-cross-diff` over 20 712 corpus cells.

### Benchmarks

`criterion` benches, all against the shared corpus:

| bench | measures |
|---|---|
| `parse` | `zebra2.ein`, `zebra.ein`, the stdlib modules |
| `load` | parse + imports + macro expansion + index build |
| `saturate_root` | root saturation only |
| `match_hot` | `match::run` over the saturated root, per plan |
| `boundary` | a full `admit_from_boundary` round — **both puzzles** |
| `solve_fast` / `solve_exhaustive` | end-to-end, both puzzles |
| `fork` | fork + first delta write |

**Eight names, nine cases since [S1a.6.1](../README.md#s1a61--fresh-profile-and-bench-baseline).**
`boundary` runs both puzzles, and every row above says which input it times:
*a bench pair is only comparable if both halves name the same input*.

**Variance is a gate, not a footnote.** `criterion`'s console output prints no
standard deviation; `utils/criterion_table.py` reads the `estimates.json` it
leaves behind, prints mean / sd / relative sd / CI for every case, and exits
non-zero if any exceeds `--max-rsd` (3 %, S1a.6.1's threshold). That is what
the nightly "benches with regression thresholds" step should run, and it is
what a phase-internal before/after must pass before its number is quoted.

---

## 5. Repo hygiene

- `.gitignore`: `ein.rs/target/`, `corpus/out/`.
- `AGENTS.md` (= `CLAUDE.md`) gains an `ein.rs/` entry under *Where
  things live* and a note that `stdlib/` is shared. Do this in P1a.0, not
  at the end — the file is how future sessions orient.
- Commits go to `master` (repo convention), one stage per commit where
  practical, with the stage id in the subject.

## Cross-links

- [11 — Shared assets](11_shared_assets.md) — `stdlib-check`, the corpus
  manifest.
- [`run_tests.sh`](../../../../run_tests.sh) — the per-commit tier, run
  locally.
