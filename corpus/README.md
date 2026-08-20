# corpus/

**The inventory**: every `.ein` file the engine is exercised over, and the
invocations each one is exercised under.

- **`corpus.toml`** — the manifest. Generated once from the tree, maintained by
  hand thereafter. A completeness check fails on any `.ein` under `examples/`
  or `stdlib/` with no entry, so the corpus cannot silently miss a file.
- **`fuzz_findings/`** — minimised inputs a fuzzer found something on.

This directory was `conformance/` until M1a
[S1a.10.3](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.3_corpus_without_an_oracle.md).
The name went with the thing it named: `conformance/` meant *two
implementations agreeing*, and the manifest is what survived the second engine
leaving the tree ([P1a.10](../plans/m1a_rust/p1a.10_single_implementation/README.md)).
The `--tier T0…T3` vocabulary, the `ein-conformance` runner and the
`--impl-a` / `--impl-b` pair went with it too; nothing below defines a tier,
and a plan document that mentions one is describing 2026.

## Who reads it

Everything is `cargo test`; nothing shells out to a second engine.

| reader | what it does with the manifest |
|---|---|
| [`ein-cli/tests/corpus_cli.rs`](../ein.rs/crates/ein-cli/tests/corpus_cli.rs) | **the sweep** — runs every entry under every declared run, as a process, and holds each cell's exit code to a banked golden |
| [`ein-corpus/src/manifest.rs`](../ein.rs/crates/ein-corpus/src/manifest.rs) | the completeness check and the manifest's own invariants — nine tests |
| [`ein-render/tests/corpus_shapes.rs`](../ein.rs/crates/ein-render/tests/corpus_shapes.rs) | digests every observable surface of every corpus *file* (4 228 renderings), which is a superset of what the runs reach |
| [`ein-render/tests/id_order_invariance.rs`](../ein.rs/crates/ein-render/tests/id_order_invariance.rs) | runs the same sweep twice under a permuted id space |
| [`ein-cli/tests/summary_properties.rs`](../ein.rs/crates/ein-cli/tests/summary_properties.rs) | the counter identities, over every `solve` cell |

The last three walk the *files* (`ein_corpus::corpus_files`) rather than the
manifest's rows, because their subject is a surface rather than an invocation.
The completeness check is what keeps the two views the same set.

```sh
cargo test --manifest-path ein.rs/Cargo.toml -p ein-cli --test corpus_cli
EIN_CORPUS_SLOW=1 cargo test … --test corpus_cli   # the slow entries too
EIN_BLESS=1      cargo test … --test corpus_cli    # re-bank the exit golden
```

## Manifest format

```toml
schema = "ein-corpus/2"

[[entry]]
path   = "examples/zebra2.ein"     # repo-root-relative
group  = "positive"
runs   = ["solve", "solve -e", "saturate", "render rules"]
levers = ["-L", "-K"]              # each makes one more `solve <lever>` run
slow   = true                      # excluded from the default sweep
note   = "why this entry is interesting, when it is not obvious"
```

A **run name is the `ein` argv with the file position elided**: `"solve -e"` is
`ein solve <path> -e`, `"render rules"` is `ein render rules <path>`. Two
substitutions happen in the sweep:

- `{out}` expands to that cell's output directory, so a run can name its own
  artefacts — `"solve --trace {out}/trace.md"`, `"solve --dump-states {out}/states"`;
- every `solve` run silently gains `--json-summary {out}/summary.json`.

`runs` is **the invocations this entry is exercised under**. Until
S1a.10.3 it read "…*compared* under", and the difference is the whole of that
stage: a run is now a thing that must work, not a thing two engines must agree
about.

## Groups

| group | what it holds | the sweep expects |
|---|---|---|
| `positive` | `examples/**/*.ein` outside `broken/` and `ein-bugs/` | at least one run answers; catalogued in [`examples/README.md`](../examples/README.md) |
| `stdlib` | the [stdlib](../stdlib/) modules, loaded standalone | as `positive` — it exercises the import + macro machinery on its own terms |
| `parse-negative` | `examples/broken/*.ein` | every run refused, `IRParseError` with `file:line:col` |
| `load-negative` | `examples/broken/load/*.ein` | parse, then fail to load; the exact message is checked in beside each fixture ([README](../examples/broken/load/README.md)) |
| `compile-negative` | `examples/broken/compile/*.ein` | parse and load, then the compiler refuses; `.expected` beside each ([README](../examples/broken/compile/README.md)). `activator_arity.ein` sits in that directory and is `positive`: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
| `regression` | `examples/ein-bugs/*.ein` | **nothing uniform** — see below |
| `generated` | [`utils/fuzz_ein.py`](../utils/fuzz_ein.py) output | empty in the manifest, used at run time |

A negative fails whichever way you enter, so what varies is *which entry point
reports it*: the negative groups run `solve`, `saturate` and `render rules` —
three presentations of one error, and three chances to format one of them
differently. With one caveat the sweep measured and this table did not use to
admit: **`render rules` does not load the KB**, so ten of the thirty
load-negatives render their rules and exit 0. That is not a hole, it is where
the pass boundary is; the exit golden records it per cell.

`regression` is the group with no rule, and deliberately. It holds the inputs
that once broke an implementation — a `sorted()` over mixed types
([D2](../plans/m1a_rust/divergences.md#d2--sortedalive-raises-in-einpy-where-einrs-answers)),
a goal binding that was a JSON number, an `(or …)` whose arms bind different
variables — and what "correct" means for one of them is whatever it does now.
Seven of its ten entries answer on every run, one answers under `saturate` and
is refused under `solve`, two are refused outright — and the exit golden is the
only statement of which. It was called `crash-parity` until S1a.10.3, when the
claim it encoded (*ein.py raises here*) lost its subject.

`generated` is empty rather than absent because the question it names is open:
`utils/fuzz_ein.py` files each generated case that loads under this group, and
whether it keeps doing so is
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md)'s.
A `golden` group was empty here for the same reason until
[Q-M1a.9](../plans/m1a_rust/open_questions.md#q-m1a9--where-do-goldens-live)
was answered — goldens live in `ein.rs/crates/<crate>/tests/golden/` — and it
is gone rather than empty now, because an empty group is a question with a
home and an answered question is not one.

## Dropped runs

A handful of entries carry a `note` saying which runs they do *not* have,
because the run outlived a 150 s budget under CPython. Two shapes:

- **`saturation/**`** demos exist to show ONE rule firing, and none of them
  bounds a hypothesis space. Where `solve` does not finish there, it is
  measuring the blind enumerator against a domain the demo never set — the
  same trap `features/04_open` and `features/05_stdlib_domain_elim` fall into.
- **`zebra2-minus-15`** is the honest case: genuinely under-determined, so its
  exhaustive search is large rather than pathological.

A run nobody can finish is not coverage. **The budget these were dropped
against was CPython's**, and ein.rs is 30–200× faster on the same workloads
([baseline.md](../plans/m1a_rust/p1a.6_performance/baseline.md)), so some of
them are now affordable. Re-measuring them is a corpus growth item, not a
deletion: the notes stay until a run replaces them.

## `slow`

17 entries, 118 cells, excluded from the sweep unless `EIN_CORPUS_SLOW=1`. The
whole default sweep is **542 cells in ~3 s**; the slow entries alone are two
minutes of it, and a gate that takes two minutes to say the same thing is a
gate people learn to skip.

## Levers

`levers` is the `SolverConfig` on/off matrix from
[`utils/feature_matrix.py`](../utils/feature_matrix.py), restricted to what the
CLI can express: `-L` (no lookahead), `-K` (no kill cache), `-y` (lattice
sanity check), `-o score-sum` (lattice order). The other six levers that matrix
drives are reachable only through the library API or a puzzle's own
`(config …)` block, so the sweep — which runs a process — cannot flip them.
Tracked as [Q-M1a.16](../plans/m1a_rust/open_questions.md#q-m1a16--how-does-the-harness-drive-the-lever-matrix).

## Growth rule

Any defect found outside the corpus becomes a corpus entry in the same commit
that fixes it. The corpus only grows.
