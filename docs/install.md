# Installing `ein`

**Two channels, and those are the two:** a binary from a release, or
`cargo install` from a checkout. There is no `pip install` — see
[§ There is no Python package](#there-is-no-python-package).

`ein` is one self-contained executable. The `std.*` standard library is
compiled into it ([`stdlib/`](../stdlib/README.md)), so a downloaded binary
solves `zebra2.ein` on a machine with nothing else on it — which is what
[§ Verify what you got](#verify-what-you-got) is for: the one thing that can
differ between a binary and a checkout beside it is *which* stdlib a run
reads, and `ein --version` answers that in one line.

> **Written at M1a [S1a.9.3](history/m1a_rust/README.md#s1a93--packaging-and-release).**
> The release workflow that produces the artefacts below
> ([`.github/workflows/release.yml`](../.github/workflows/release.yml)) is
> written and reviewed; **the first tag is what runs it**. Until one is
> pushed, § Build from source is the channel that has been exercised.
>
> **Still true on 2026-08-29, and now a decision rather than a status.** M1e
> [S1e.1.6](../plans/m1e_review_processing/p1e.1_open_questions/s1e.1.6_coverage_gaps.md)
> (the review's `Q10`) weighed running the matrix on a throw-away pre-release
> tag against waiting, and **accepted**: the workflow's `publish` job creates
> a **public GitHub release**, so pushing a tag is a decision about what this
> project has shipped and not a test anyone may run on its behalf. Four
> platform legs, the `--jobs` cross-diff and the `--no-default-features` leg
> are therefore still unexercised, and this paragraph is the whole of the
> evidence for the table below. One command changes that — `git tag v0.0.0-rc1
> && git push origin v0.0.0-rc1` — and it is the maintainer's to run.

---

## Download a binary

Releases carry one executable per platform plus `SHA256SUMS`:

| file | platform | notes |
|---|---|---|
| `ein-linux-x86_64` | Linux x86_64 | glibc ≥ 2.35 (built on Ubuntu 22.04) |
| `ein-linux-aarch64` | Linux aarch64 | same |
| `ein-macos-universal2` | macOS | one file, both architectures |
| `ein-windows-x86_64.exe` | Windows x86_64 | |
| `ein-linux-x86_64-musl` | Linux x86_64 | **statically linked**, best-effort; see below |

```sh
curl -LO https://github.com/agutikov/ein/releases/latest/download/ein-linux-x86_64
curl -LO https://github.com/agutikov/ein/releases/latest/download/SHA256SUMS
sha256sum -c SHA256SUMS --ignore-missing
chmod +x ein-linux-x86_64 && ./ein-linux-x86_64 --version
```

**The musl build is a fallback, not the default.** It is there for a system
whose glibc is too old to run the others at all. Building `snmalloc` needs a
C++ toolchain targeting musl, which the release runner does not have, so that
artefact is `--no-default-features`: the system allocator, no `.einb`
container, no `--jobs` fan-out. It costs a measured **+25 % on `solve
zebra2.ein -e`** and **+36 % on `solve zebra.ein -e`**
([`feature_cost.md` §2](history/m1a_rust/measurements/feature_cost.md#2---no-default-features--what-a-packager-is-buying)).
Take the glibc binary unless glibc is the problem.

## `cargo install`

The workspace lives in `ein.rs/`, one directory below the repository root, so
there is no manifest where `cargo install --git` looks for one. Clone, then
install by path:

```sh
git clone https://github.com/agutikov/ein
cargo install --path ein/ein.rs/crates/ein-cli --locked
ein --version
```

This builds with default features, which needs **`cmake` and a C++ compiler**
(`snmalloc` is vendored and worth 8–16 % of a solve). Without them:

```sh
cargo install --path ein/ein.rs/crates/ein-cli --locked --no-default-features
```

— and read [`feature_cost.md`](history/m1a_rust/measurements/feature_cost.md)
first, because that build also drops `ein kb` and `--jobs`.

`cargo install` puts `ein` on `$PATH`. That is right for using ein as a tool
and **wrong if you are working on ein** — see
[§ If you are developing ein](#if-you-are-developing-ein).

## Build from source

```sh
./build.sh                          # the Rust workspace (release) + the C baselines
ein.rs/target/release/ein --version
```

`./build.sh --no-snmalloc` builds against the system allocator and needs
neither `cmake` nor a C++ compiler; `./build.sh --engine` skips the C
baselines. The toolchain is pinned by
[`ein.rs/rust-toolchain.toml`](../ein.rs/rust-toolchain.toml); the gate is
`./run_tests.sh`, which additionally needs **Graphviz** on `PATH`.

---

## Verify what you got

`ein --version` reports four things a version number does not:

```text
$ ein --version
ein 0.1.0
protocol   ein-events/1
container  einb/1.0
features   einb, parallel, snmalloc
stdlib     sha256:a498c7…1d79  embedded
```

| line | what it answers |
|---|---|
| `ein <semver>` | the engine version — one number for the whole workspace |
| `protocol` | the schema a `--events` file declares on its first line |
| `container` | the `.einb` format major.minor this build reads and writes, or `none` |
| `features` | what was compiled in. **`--jobs N` is accepted and inert without `parallel`**, and `ein kb` is not registered without `einb` — neither absence has an error message, so this line is where they are visible |
| `stdlib` | the SHA-256 of the `std.*` manifest **this binary will load**, and which of the three resolution steps found it |

The stdlib line is the one worth reading twice. `std.*` is resolved at run
time in three steps — `$EIN_STDLIB`, a `stdlib/` directory found by walking up
from the executable, then the copy compiled into the binary — so the *same*
binary can load different programs depending on where it is run from. The
digest is over `stdlib/MANIFEST.sha256` as resolved, printed whole, so the
check is a command you already have:

```sh
ein --version | grep stdlib          # sha256:a498c7…  checkout /path/to/ein/stdlib
sha256sum /path/to/ein/stdlib/MANIFEST.sha256
```

Equal digests mean the binary and that checkout agree about the standard
library. A binary reporting `embedded` while you are editing a checkout's
`stdlib/` is reading neither what you edited nor an error message, and that is
the class of confusion this line exists to end in one command.

## Point it at a different stdlib

```sh
EIN_STDLIB=/path/to/stdlib ein solve puzzle.ein
```

`$EIN_STDLIB` wins over both other steps, silently — which is why
`ein --version` names it rather than printing a bare path.

**The marker rule applies to the override too, since M1e S1e.3.5.** A
`stdlib/` directory is the stdlib only if it contains `MANIFEST.sha256` — for
the walk, which keeps going past one that does not, and now for
`$EIN_STDLIB`, which used to be taken as given with no check at all. Point it
somewhere that cannot prove itself and the first `(import std.…)` says so, by
name:

```
kb load error: (import std.algebra) — $EIN_STDLIB names /tmp/x, which has no
MANIFEST.sha256 — a directory is the stdlib only if it carries the marker …
```

where it used to say *module not found at /tmp/x/algebra.ein* — true, and a
sentence that names the module rather than the variable that chose the
directory. That was
[EH-M2](../plans/m1e_review_processing/README.md#the-findings), and what it
cost was the diagnosis rather than the answer.

Two things the check deliberately does not do. It is asked at the **first
`std.*` import**, so a program that imports nothing from the stdlib is not
refused for the shape of a variable it never reads. And `ein --version` does
**not** consult it: that line still prints `stdlib     unreadable  $EIN_STDLIB
<path>` and keeps going, because a version line that refused to render would
be a worse way to learn the same thing.

**The walk still prefers whatever checkout it lands under**, and that is a
decision rather than an oversight. A binary copied under an unrelated tree
containing `stdlib/MANIFEST.sha256` will read *that* stdlib. The obvious
guard — warn when the resolved manifest differs from the copy compiled into
the binary — was considered at S1e.3.5 and **refused**, because that mismatch
is the *normal* state of stdlib development: the checkout tier exists so an
edited module takes effect with no rebuild, so the warning would fire on the
working case, which is how a warning gets turned off. The instrument for the
hazard is the one above: `ein --version` names the step and the path, in one
command.

`$EIN_STDLIB` is one of **eight** environment variables the shipped binary
reads, and the only one a normal install needs. The whole set, with the three
classes of name that are *not* that —
[`docs/kernel/configuration.md` § The environment](kernel/configuration.md#4-the-environment).

## There is no Python package

The `ein` name on PyPI is unclaimed and this milestone does not claim it. The
PyO3 binding was **deferred on 2026-08-21** for want of a consumer —
[M20](../plans/m20_gui/README.md) links the crates, [M10](../plans/m10_external_benchmarks/README.md)'s
benchmark runner must shell out to be a fair measurement, and
[M2](../plans/m2_nl_to_ir/README.md)'s reason turned out to be an HTTP server.
[Q-M1a.23](history/m1a_rust/open_questions.md#q-m1a23--when-does-the-engine-need-a-python-binding)
holds the three conditions that would revive it.

To drive ein from Python today, run the binary and read its output: `ein solve
… --json-summary out.json` for the verdict and counters, `--events e.jsonl`
for the step-by-step narration
([the protocol](kernel/inference/events.md)). To embed it in a Rust program,
link the crates.

## If you are developing ein

**Do not put `ein` on `$PATH`.** Every script under [`utils/`](../utils/README.md)
names the binary it runs — `$EIN_BIN`, or `--bin`, defaulting to
`ein.rs/target/release/ein` — precisely so that a measurement says which build
produced it. An ambiguous `ein` on `$PATH` turns "the number went up" into a
question about which binary answered, and that has burned every project that
allowed it.

```sh
./build.sh                                    # then
EIN_BIN=ein.rs/target/release/ein utils/bench_env.sh python3 utils/e2e_baseline.py
```

## Cross-links

- [`README.md`](../README.md) — what ein is, and the quickstart
- [`docs/guide/`](guide/) — the tutorial: learn ein by solving the Zebra puzzle
- [`stdlib/README.md`](../stdlib/README.md) — what `std.*` contains
- [M1a § P1a.9 — Release](history/m1a_rust/README.md#p1a9--release) — the
  release phase as history: what is measured, and what only a tag can prove
