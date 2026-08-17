# conformance/

The parity corpus: which files the two `ein` implementations are compared on,
and under which runs.

- **`EVENTS.md`** — the `--events` protocol, the T2 parity surface.
- **`corpus.toml`** — the manifest. Generated once from the tree, maintained by
  hand thereafter. A completeness check in both test suites fails on any `.ein`
  under `examples/` or the stdlib that has no entry, so the corpus cannot
  silently miss a file.
- **`out/`** — artefacts from the last `ein-conformance run` (git-ignored; the
  runner wipes it at the start of every run, because a stale tree would let a
  run that wrote nothing look like a run that wrote the same thing as before).

The runner lives at [`ein.rs/crates/ein-conformance`](../ein.rs/crates/ein-conformance);
the contract it enforces is [`plans/m1a_rust/design/01_parity_contract.md`](../plans/m1a_rust/design/01_parity_contract.md).

```sh
cd ein.rs && cargo build --release
# Python vs Python — the P1a.0 acceptance gate.
./target/release/ein-conformance run \
    --impl-a "python3 -m ein.cli" --impl-b "python3 -m ein.cli" --tier T3
# The determinism sweep: one implementation, two hash seeds.
./target/release/ein-conformance run \
    --impl-a "python3 -m ein.cli" --impl-b "python3 -m ein.cli" \
    --env-a PYTHONHASHSEED=0 --env-b PYTHONHASHSEED=42 --tier T3
# One fixture, one difference, by hand — how the tool actually gets used.
./target/release/ein-conformance run … --filter zebra2 --tier T2 -v
./target/release/ein-conformance diff a.jsonl b.jsonl --classes
```

Tiers: **T0** the verdict, **T1** every counter, **T2** the ordered event log,
**T3** byte-identical artefacts. T0/T1 read the `summary.json` each `solve`
cell is given (`--json-summary`); T2 adds `--events … --events-level verbose`
to each `solve` / `saturate` cell and compares the two logs structurally; T3
compares stdout, stderr, the exit code and every produced file. The
subsumption is mechanical rather than claimed — `summary.json` is one of the
files T3 compares, so a T3 pass cannot hide a T1 difference.

Both implementations run with the repo root as their working directory and are
addressed by explicit path, so there is never a question of which `ein` ran.
`ein.rs` is deliberately **not** installed onto `$PATH` during the port.

## Manifest format

```toml
schema = "ein-corpus/1"

[[entry]]
path   = "examples/zebra2.ein"     # repo-root-relative
group  = "positive"
runs   = ["solve", "solve -e", "saturate", "render rules"]
levers = ["-L", "-K"]              # each makes one more `solve <lever>` run
slow   = true                      # nightly tier only
note   = "why this entry is interesting, when it is not obvious"
```

A **run name is the `ein` argv with the file position elided**: `"solve -e"` is
`ein solve <path> -e`, `"render rules"` is `ein render rules <path>`. Two
substitutions happen in the runner:

- `{out}` expands to that cell's output directory, so a run can name its own
  artefacts — `"solve --trace {out}/trace.md"`, `"solve --dump-states {out}/states"`;
- every `solve` run silently gains `--json-summary {out}/summary.json`, which is
  what the T0 and T1 tiers read, and at `--tier T2` every `solve` / `saturate`
  run also gains `--events {out}/events.jsonl --events-level verbose`. Neither
  is worth making each manifest entry ask for.

## Groups

| group | what it holds | notes |
|---|---|---|
| `positive` | `examples/**/*.ein` outside `broken/` | the feature surface; catalogued in [`examples/README.md`](../examples/README.md) |
| `parse-negative` | `examples/broken/*.ein` | `IRParseError` with `file:line:col` |
| `load-negative` | `examples/broken/load/*.ein` | parse, then fail to load; the exact message is checked in beside each fixture ([README](../examples/broken/load/README.md)) |
| `stdlib` | the stdlib modules, loaded standalone | exercises the import + macro machinery on its own terms |
| `golden` | the artefacts `ein.py/tests/golden/**` pins | **empty until Q-M1a.9** decides where goldens live; the fixtures that produce them are already `positive` entries |
| `generated` | `examples/gen_zebra2_variants.py` output | **empty**: the variants are generated, not checked in, so they join the nightly tier when the generator is wired to the harness |
| `crash-parity` | inputs where ein.py raises an unhandled exception | compared by **exit code + exception class**, never by message — see below |

A negative fails whichever way you enter, so what varies is *which entry point
reports it*: both negative groups run `solve`, `saturate` and `render rules` —
three presentations of one error, and three chances for a port to format one of
them differently.

Two groups are deliberately empty rather than absent: each names a decision
that is open, and an empty group in the manifest is a question with a home,
where a missing one is a question nobody wrote down.

### crash-parity

Q-M1a.14 proposed comparing "exit code + the first line of stderr". The first
`crash-parity` fixture ruled that out. `examples/ein-bugs/mixed-type-hypothesis.ein`
dies in `apriori.layer_1` with

```
TypeError: '<' not supported between instances of 'int' and 'str'
```

…and *which operand is named first* depends on the `frozenset` iteration order
inside `sorted`, so ein.py alternates between two messages across
`PYTHONHASHSEED` values. Comparing the line would make the determinism sweep
fail on a difference that is not one. The group compares the exit code and the
exception class, and the message body is normalised away.

## Dropped runs

A handful of entries carry a `note` saying which runs they do *not* have,
because the run outlives a 150 s budget under CPython. Two shapes:

- **`saturation/**`** demos exist to show ONE rule firing, and none of them
  bounds a hypothesis space. Where `solve` does not finish there, it is
  measuring the blind enumerator against a domain the demo never set — the
  same trap `features/04_open` and `features/05_stdlib_domain_elim` fall into.
- **`zebra2-minus-15`** is the honest case: genuinely under-determined, so its
  exhaustive search is large rather than pathological.

A run nobody can finish is not coverage, and leaving it in would make the
nightly tier report "timed out on both sides" forever — which trains the
reader to skip that line. Recorded per entry so the gap is a decision.

## Levers

`levers` is the `SolverConfig` on/off matrix from
[`utils/feature_matrix.py`](../utils/feature_matrix.py), restricted to what the
CLI can express: `-L` (no lookahead), `-K` (no kill cache), `-y` (lattice
sanity check), `-o score-sum` (lattice order). The other six levers that matrix
drives are reachable only through the Python API or a puzzle's own
`(config …)` block, so the harness — which shells out — cannot flip them.
Tracked as **Q-M1a.16**.

## Growth rule

Any divergence found outside the corpus becomes a corpus entry in the same
commit that fixes it. The corpus only grows.
