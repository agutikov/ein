# S1a.9.3 — Packaging and release

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 2 days
**Depends on:** [S1a.9.0](s1a.9.0_slow_corpus.md)
**Implements:** [design/12](../design/12_toolchain_and_layout.md) §4
(release tier)

> **Shipped 2026-08-23.** Six tasks, five of them done and verifiable here;
> the sixth is done and verifiable only on a tag — see
> [§ What CI has not yet proved](#what-ci-has-not-yet-proved), which the
> workflow's own header points at so that a green badge is read for what it
> is. What the stage found on the way is the part worth keeping:
> **two of the three features `--no-default-features` was documented to drop
> were not being dropped**, and one of them does not exist
> ([`feature_cost.md`](feature_cost.md)).
>
> **Amended 2026-08-21 — no wheels.** The phase's two binding stages are
> deferred (see the [phase README](README.md)'s scope change and
> [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)),
> so there is no PyO3 module to package. What is left is the half of this
> stage that was always the point: **one binary, three platforms, and a
> release that cannot ship a red gate.** The wheel task, the `maturin` matrix
> and the `ein_rs.__version__` tie-in are gone; `pip install` is not one of
> the channels this milestone ships.

## Context

Distribution was reason #1 for the port — "M20 GUI + M2 NL frontend ship
to end-users; PyPy adds a second interpreter for the user to install,
ein.rs ships as a single binary". This stage cashes that in: static
binaries for three platforms and a release process that cannot ship an
engine whose gate is red.

The argument is *stronger* without the wheel, not weaker. "PyPy adds a
second interpreter for the user to install" was the complaint; shipping a
Python extension module as a headline channel would have re-introduced the
interpreter the port existed to remove.

## Acceptance

- `ein` binaries for Linux (x86_64 + aarch64, statically linked against
  musl where practical), macOS (universal2), and Windows (x86_64), each
  sweeping the corpus on its own platform —
  `cargo test -p ein-cli --test corpus_cli` with `EIN_CORPUS_SLOW=1`, which
  is what "the corpus at T3" means after
  [S1a.10.3](../p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md).
- `ein --version` reports engine version, protocol version, feature
  flags, and the stdlib manifest hash.
- Release artefacts carry checksums; the release job refuses to publish if
  `cargo test --workspace`, the slow corpus sweep or the acceptance gate is
  red.
- A `--no-default-features` build still compiles and passes the unit
  suite (proving the feature gating is real).

## Tasks

### Task T1a.9.3.1 — Build matrix

Cross-compile targets, the musl static build, macOS universal2, and the
Windows toolchain. Keep the embedded stdlib
([S1a.0.3](../p1a.0_conformance_harness/s1a.0.3_shared_stdlib_and_examples.md)
T1a.0.3.4) working on every target — `include_dir!` paths are the usual
casualty of a cross build.

### Task T1a.9.3.2 — The `--no-default-features` build

The feature gating has to be real, and this is where that gets proved rather
than asserted: a build without `snmalloc`, without `events`, without `einb`
compiles, passes the unit suite, and is what a distro package that would
rather not vendor a C++ allocator actually builds. It costs 15.9 % of `solve
zebra2 -e` ([design/12](../design/12_toolchain_and_layout.md) §3) and that
number belongs in the release notes so a packager can make the trade
knowingly.

### Task T1a.9.3.3 — Platform conformance

Run the corpus at T3 on each platform, not just Linux. The likely
offenders are line endings on Windows, filesystem case-sensitivity on
macOS, and locale-dependent float or string formatting — all of which
would be silent on a single-platform CI.

### Task T1a.9.3.4 — Version reporting

`ein --version` reports the tuple. Include the stdlib manifest hash: it is the one input that can differ
between a binary and a checkout, and a mismatch explains a whole class of
confusing behaviour instantly.

### Task T1a.9.3.5 — Release workflow

Tag → build matrix → the full gate (`cargo test --workspace`, then the slow
corpus sweep) → acceptance gate → sign and publish. The ordering matters:
artefacts are built first (so a failure is diagnosable) but published only
after the gate is green.

### Task T1a.9.3.6 — Install docs

A short install section for each channel — **binary download and `cargo
install`, and those are the two** — including how to point at a different
stdlib (`EIN_STDLIB`) and how to verify a binary against the manifest. If a
reader arrives looking for `pip install`,
[Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
is the answer and the install page should link it rather than stay silent.

## What shipped

| task | | |
|---|---|---|
| T1a.9.3.1 | Build matrix | four required legs in [`release.yml`](../../../.github/workflows/release.yml) — Linux x86_64 / aarch64 (glibc, on the *oldest* runner image, because the glibc a binary links is the floor of the systems it runs on), macOS universal2 (two slices, `lipo`), Windows MSVC — plus a best-effort musl leg. **Every required leg is host == target**, which is what lets each one *run* the corpus sweep rather than only link, and which is why the task's named hazard (`include_dir!` losing the embedded stdlib under a cross build) does not arise on three of the four |
| T1a.9.3.2 | The `--no-default-features` build | it built and passed before this stage; what it did **not** do was drop what it claimed to. [`feature_cost.md`](feature_cost.md) |
| T1a.9.3.3 | Platform conformance | the corpus at T3 on every leg, and [`.gitattributes`](../../../.gitattributes), which is the difference between that meaning something on Windows and being a checkout artefact |
| T1a.9.3.4 | Version reporting | `ein --version`, ten tests |
| T1a.9.3.5 | Release workflow | `publish` `needs:` the gate, so a red gate cannot ship a binary |
| T1a.9.3.6 | Install docs | [`docs/install.md`](../../../docs/install.md) |

### T1a.9.3.2 — the feature gating was not real, and one feature was fiction

The task said *the feature gating has to be real, and this is where that gets
proved rather than asserted.* Proving it took three findings:

1. **`--no-default-features` still linked `rayon`.** `ein-cli` took
   `ein-infer` with its defaults, so the dependency-light binary carried a
   work-stealing pool it had no flag to reach. The fix settled on one rule —
   **a crate that forwards `jobs` forwards `parallel`** — which gives
   `ein-render` a default-on feature of its own (its `shape.rs` hands a job
   count to `SolveOptions`), leaves `ein-einb` without one, and makes
   `ein-cli`'s `--no-default-features` the single flag that turns the pool
   off.
2. **There is no `events` feature.** [design/12
   §3](../design/12_toolchain_and_layout.md#3-feature-flags) had reserved one
   since P1a.0 and nothing ever added it. Priced before deciding, with the
   strongest possible version of the feature: **+3.9 % / +1.8 %** — *slower*
   — so it is recorded as not taken up rather than built.
3. **The documented cost was four days stale.** 15.9 % → **+25.2 %** on `solve
   zebra2 -e`, because the engine got faster and allocation did not.

And the reason a fix here needed a test: dropping `ein-infer`'s defaults from
`ein-render` would have made `jobs_invariance` — 20 712 cells of P1a.7's
acceptance — compare `--jobs 1` against itself and stay green.
`the_sweep_is_not_vacuous` is what makes that a failure.

**Compiling is not the claim, and the workflow says so in the shape of the
check**: `cargo tree -p ein-cli --no-default-features` must contain no
`snmalloc-rs`, `ein-einb`, `blake3` or `rayon`, and the default build must
contain all four. Absence is what a packager is buying.

### T1a.9.3.4 — what `ein --version` reports, and why each line

```text
ein 0.1.0
protocol   ein-events/1
container  einb/1.0
features   einb, parallel, snmalloc
stdlib     sha256:a498c7…1d79  checkout /home/user/work/ein/stdlib
```

The stdlib line is the one the task named — *the one input that can differ
between a binary and a checkout* — and it carries **which of the three
resolution steps answered**, not only a path: an installed binary reports
`embedded`, the same binary run inside a checkout reports `checkout …`, and
those are two different programs. SHA-256 rather than the BLAKE3 `.einb`
already carries, because the verification instruction has to be a command the
reader has: `sha256sum stdlib/MANIFEST.sha256`.

The features line earns its place on `parallel` alone. `--jobs 8` in a build
without the pool is **accepted and inert** — no warning, no error — so a
machine that ignores it is otherwise indistinguishable from a machine that is
busy.

`--version` is intercepted in `run()` before `clap`, in first position only,
the same arrangement `saturate` has and for the same reason: the top-level
command requires a subcommand. It is registered on the parser anyway so
`ein --help` lists it, and `ein render rules --version` stays the usage error
it was.

### T1a.9.3.3 — `.gitattributes`, which the corpus sweep needed first

Sweeping the corpus on Windows means comparing bytes: 37 `.expected` files,
the DOT and trace goldens, `corpus_shapes.md5`, and `stdlib/MANIFEST.sha256`,
which is a SHA-256 over every `std.*` module's *contents*. Git for Windows
installs with `core.autocrlf=true`, so without
[`.gitattributes`](../../../.gitattributes) the first Windows run would have
been a wall of red that said nothing about Windows. The tree is all-LF today
and has no file git detects as binary, so `* text=auto eol=lf` costs nothing
and settles it. The other two hazards the task named were checked and are
absent: no two tracked paths differ only in case (macOS), and Rust's
formatting is not locale-sensitive.

## What CI has not yet proved

This is the stage's honest edge, and it is stated here rather than left for a
reader to infer from a badge.

**Verified on this machine** (2026-08-23, x86_64 Linux): `ein --version` and
its ten tests; `cargo test --workspace` at 619 tests; the
`--no-default-features` build, its 610-test suite and both dependency-graph
assertions; `cargo install --path ein.rs/crates/ein-cli` producing a binary
that solves `zebra2.ein` outside the checkout off its embedded stdlib;
[`feature_cost.md`](feature_cost.md)'s three arms.

**Not run anywhere yet**: every leg of the build matrix. This repository is
developed on x86_64 Linux, has never been built for aarch64, macOS, Windows or
musl, and **the first `v*` tag is what runs all four for the first time.**
There is no way to shorten that from here — a cross-build without a runner
proves the linker was willing, not that the corpus sweeps — so what the stage
can do instead is make the first tag *diagnostic*:

- each leg sweeps the corpus at T3 **on its own platform** before it uploads
  anything, so a failure names a cell rather than a checksum;
- `.gitattributes` removes the one failure mode that would have made Windows
  fail for a reason unrelated to Windows;
- `static-linux` is `continue-on-error` and outside `publish`'s `needs:`,
  because snmalloc has no C++ toolchain for musl on the runner and a
  best-effort artefact must not be able to block a release;
- the release notes are generated from what is actually staged, so a missing
  static binary is a stated absence.

The likeliest first-tag failures, in order: musl (no C++ toolchain — expected,
and handled), `ubuntu-22.04-arm` availability, then Windows path handling in
the corpus fixtures.

## Notes

- Do not install the `ein` binary onto `$PATH` in developer setups by
  default. The reason used to be the conformance harness, which invoked
  both engines by explicit path; the reason that outlives it is that
  `utils/` names its binary (`$EIN_BIN` / `--bin`) precisely so a
  measurement says which build it measured, and an ambiguous `ein` on
  `$PATH` has burned every project that allowed it.
- **The `ein` PyPI name is free and this stage does not claim it.** It came
  free when the Python package left the tree at
  [S1a.10.5](../p1a.10_single_implementation/s1a.10.5_removal.md), and the
  reflex to reserve it should be resisted: a name claimed before there is
  something to publish under it is a name that gets renamed, and the last
  observation of the old note still holds — easier before a first release
  than after. If [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
  trips, naming is that work's first decision and it will be made with a
  module in hand.
