# S1a.9.3 — Packaging and release

**Phase:** P1a.9 (Bindings and release)
**Estimate:** 2 days
**Depends on:** [S1a.9.0](s1a.9.0_slow_corpus.md)
**Implements:** [design/12](../design/12_toolchain_and_layout.md) §4
(release tier)

> **Amended 2026-08-21 — no wheels.** The phase's two binding stages are
> deferred (see the [phase README](README.md)'s scope change and
> [Q-M1a.23](../open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)),
> so there is no PyO3 module to package. What is left is the half of this
> stage that was always the point: **one binary, three platforms, and a
> release that cannot ship a red gate.** The wheel task, the `maturin` matrix
> and the `ein_rs.__version__` tie-in are gone; `pip install` is not one of
> the channels this milestone ships.

## Context

Distribution was reason #1 for the port — "M1b GUI + M2 NL frontend ship
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
