# What a feature flag costs — the release build, priced

**Stage:** [S1a.9.3](../README.md#s1a93--packaging-and-release) T1a.9.3.2 · **taken 2026-08-23**,
`master` @ `8712199`, `utils/bench_env.sh python3 utils/e2e_baseline.py --bin`.

> Intel i9-14900HX, pinned to `cpu4` (P-core, max 5 600 MHz), governor
> `powersave`, turbo on, loadavg ~1.5. Best of 7 (§2) and best of 15 (§1),
> spread printed beside every cell. **Every number here is live** — one
> engine, three builds of it, re-takeable with the two commands in §4.

T1a.9.3.2 asks for a `--no-default-features` build that "compiles and passes
the unit suite", and gives the reason: *the feature gating has to be real, and
this is where that gets proved rather than asserted.* It compiled and it
passed — and two of the three features
[design/12 §3](../design/12_toolchain_and_layout.md#3-feature-flags) says that
build drops were not being dropped at all.

| claim, before this stage | what was true |
|---|---|
| `--no-default-features` builds "against the system allocator … which a distro package that would rather not vendor a C++ allocator wants" | true, and it **still linked `rayon`** — `ein-cli` took `ein-infer` with its defaults, so the dependency-light binary carried a work-stealing pool it could not reach |
| the `events` feature "compiled out entirely when off, for a zero-overhead measurement build" | **there is no `events` feature.** The row had been in design/12 since P1a.0 and nothing ever added it; the `--events` emitter is unconditional |
| `--no-default-features` "costs 15.9 % of `solve zebra2 -e`" | measured 2026-08-19 against an engine that took 98.3 ms on that cell. It takes 30.1 ms now, and the allocator's share grew with everything else shrinking: **+25.2 %** |

§1 prices the feature that does not exist, §2 the one that was not reachable,
§3 says what changed as a result.

---

## 1. The `events` guard — **not worth a feature**

Every emit site in the engine is written `if events.on() { … }`
([`events.rs`](../../../../ein.rs/crates/ein-infer/src/events.rs) § *Off is
free*), and `on()` is one discriminant compare against `Sink::Off`. design/12
reserved a feature to compile that compare away. The measurement arm is the
strongest possible version of that feature — `Events::on()` replaced by
`false`, so LLVM folds every guard and every closure behind it dies — built
into `target-noevents/` and run against the shipping binary:

| workload | shipped | guard compiled out | |
|---|---:|---:|---:|
| `solve zebra2.ein -e` | **28.94 ms** | 30.06 ms | **+3.9 %** |
| `solve zebra.ein -e` | **45.43 ms** | 46.23 ms | **+1.8 %** |

*Best of 15, spread 5.3 / 2.0 % and 4.5 / 3.9 %. At best of 7 in §2's series
the same two cells read +2.4 % and +0.4 %, so the sign reproduces and the
magnitude does not.*

**Removing the branch made it slower, twice, at two sample counts.** Not by
much and not for an interesting reason — dead-code elimination moved the
layout of a hot function and the machine liked the old one better — but that
is exactly the finding: there is no win here to give up. The guard costs
nothing measurable on a path that runs ~234 k times per exhaustive `zebra2`,
because it is a perfectly predicted compare on a value that never changes
during a run.

So the feature is **not taken up**, and design/12 §3's row says so with this
number rather than describing a build nobody can produce. The reserved-row
precedent is the same file's `zstd` and `boxcar` entries: a row that records a
decision is worth more than a row that reserves a plan.

**What this does not say.** It is not a claim that `--events FILE` is free —
that turns the sink on and writes a line per step, which costs what it costs.
It is a claim about the **off** path, which is every run that does not pass
the flag.

## 2. `--no-default-features` — what a packager is buying

Three builds of one engine, one series, 2026-08-23:

| | features |
|---|---|
| **shipped** | `snmalloc`, `einb`, `parallel` (the default) |
| **light** | none — system allocator, no container, no fan-out |
| noevents | the default, with §1's guard folded out |

| workload | shipped | light | | noevents | |
|---|---:|---:|---:|---:|---:|
| `solve zebra2.ein -e` | 30.05 ms | 37.62 ms | **+25.2 %** | 30.78 ms | +2.4 % |
| `solve zebra2.ein` | 9.08 ms | 9.64 ms | +6.2 % | 9.07 ms | −0.1 % |
| `solve zebra.ein -e` | 46.09 ms | 62.77 ms | **+36.2 %** | 46.27 ms | +0.4 % |
| `solve zebra.ein` | 10.33 ms | 12.12 ms | +17.3 % | 10.58 ms | +2.4 % |
| `render rules zebra2` | 1.54 ms | **1.01 ms** | **−34.4 %** | 1.54 ms | +0.0 % |
| `saturate zebra2` | 3.58 ms | 3.54 ms | −1.1 % | 3.57 ms | −0.3 % |

Best of 7; spread 1.1–10.5 %, the two double-digit cells being `render`, whose
1 ms is almost all process start-up. Peak RSS is 17.3 MB on all eighteen
cells.

**The trade moved, and in the direction that matters.** T1a.6.2.7 measured the
same two builds on 2026-08-19 (`66f24d5`) with the same command and the same
arm definition — `cargo build --release -p ein-cli --no-default-features` —
and got **+18.9 %** on `zebra2 -e` and **+8.1 %** on `zebra -e`. Today it is
**+25.2 %** and **+36.2 %**. Nothing about the allocator changed; the engine
did. `zebra -e` went 395.7 → 46.1 ms over P1a.6–P1a.7, and allocation is one
of the few costs those stages did not remove, so its *share* grew while its
absolute size fell (29.5 ms of it then, 16.7 ms now).

**The one cell where the light build wins is start-up, and it reproduces.**
`render rules zebra2` is 1.0 ms of which almost all is process start-up, and
snmalloc's arena set-up costs ~0.5 ms of it. That is
[baseline.md §13](baseline.md#t1a627--the-global-allocator-measured-three-ways)'s
¶ footnote, measured there at 21 samples and unchanged here four days and one
parallelism phase later — 1.1 → 1.0 ms light, 1.6 → 1.5 ms shipped. A packager
whose users run `render` and nothing else is genuinely better off without
snmalloc; everyone else repays the 0.5 ms inside the first solve.

### What `light` actually drops, now

The three features are gated for real as of this stage — `ein-cli` forwards
`parallel` and takes `ein-infer` and `ein-render` without their defaults (§3
has the rule):

```
$ cargo tree -p ein-cli --no-default-features -e normal --prefix none | sort -u
… no snmalloc-rs, no ein-einb, no blake3, no rayon …
```

The release job asserts that in **both** directions — the light build has none
of the four, the default build has all four — because "compiles without them"
was already true while "links none of them" was not, and only the second is
what a distro package is buying.

| gone with the defaults | what a user sees |
|---|---|
| `snmalloc` | the table above |
| `einb` | `ein kb` is not registered; a `.einb` path is refused by the loader that would have opened it |
| `parallel` | `--jobs N` parses and is inert — every layer runs on the committing thread |

The third is the one with no error message, which is why `ein --version` lists
the features: a build that silently ignores `--jobs 8` is otherwise
indistinguishable from a machine that is busy.

## 3. What changed because of this

- [design/12 §3](../design/12_toolchain_and_layout.md#3-feature-flags): the
  `events` row is **not taken up**, with §1's number; the `parallel` row says
  the binary forwards it; the `snmalloc` row carries §2's re-take beside the
  2026-08-19 figure it replaces, both dated.
- **A crate that forwards `jobs` forwards `parallel`.** `ein-cli` gains
  `parallel` in its default set and takes `ein-infer` and `ein-render` without
  theirs; `ein-render` gains a default-on `parallel` of its own, because
  `shape.rs` hands a job count to `SolveOptions` and a build of it without the
  pool would accept the argument and ignore it; `ein-einb` needs no row, since
  it never forwards one. `ein-render` also takes `ein-infer/parallel` as an
  unconditional **dev**-dependency — a dev-dependency is not a default
  feature, so it survives `--no-default-features` and keeps
  `jobs_invariance` asking its question in both configurations.
- `jobs_invariance` gains `the_sweep_is_not_vacuous`, which fails if the build
  it is running in has no fan-out. Without it, the change above could have
  turned 20 712 cells into 20 712 comparisons of `--jobs 1` against itself and
  stayed green.
- `.github/workflows/release.yml` asserts the dependency graph in both
  directions, and the musl leg — which has no C++ toolchain and so cannot
  carry snmalloc — quotes §2 in the release notes so nobody picks the static
  binary without knowing what it costs.

## 4. Reproducing this

```sh
# §2 — three builds of one engine
cargo build --release -p ein-cli
cargo build --release -p ein-cli --no-default-features --target-dir target-nodefault
utils/bench_env.sh python3 utils/e2e_baseline.py --runs 7 \
    --bin shipped=ein.rs/target/release/ein \
    --bin light=ein.rs/target-nodefault/release/ein

# §1 — the guard, folded out. `Events::on()` -> `false`, built aside, reverted.
#      One line, and the arm is only meaningful with the *rest* of the tree
#      identical, so build it before you revert and not after.
```

`target-nodefault/` and `target-noevents/` are git-ignored build directories
of the same shape as `target-alloc-system/` and `target-fd/`; nothing but a
measurement reads them.

## Cross-links

- [S1a.9.3](../README.md#s1a93--packaging-and-release) — the stage
- [baseline.md §13](baseline.md) — the 2026-08-19
  allocator series this re-takes one arm of
- [design/12 §2, §3](../design/12_toolchain_and_layout.md) — the dependency
  policy and the feature table
- [`corpus_cost.md`](corpus_cost.md) — the phase's other measurement, and the
  same discipline: a flag is a claim, and a claim is measured or removed
