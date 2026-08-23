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
| [`ein-cli/tests/corpus_cli.rs`](../ein.rs/crates/ein-cli/tests/corpus_cli.rs) | **the sweep** — runs every entry under every declared run, as a process, holds each cell's exit code to a banked golden, and times them: it is what holds `cost_ms` and `slow` to the wall clock |
| [`ein-corpus/src/manifest.rs`](../ein.rs/crates/ein-corpus/src/manifest.rs) | the completeness check and the manifest's own invariants — ten tests |
| [`ein-render/tests/corpus_shapes.rs`](../ein.rs/crates/ein-render/tests/corpus_shapes.rs) | digests every observable surface of every corpus *file* (4 228 renderings), which is a superset of what the runs reach |
| [`ein-render/tests/id_order_invariance.rs`](../ein.rs/crates/ein-render/tests/id_order_invariance.rs) | runs the same sweep twice under a permuted id space |
| [`ein-render/tests/jobs_invariance.rs`](../ein.rs/crates/ein-render/tests/jobs_invariance.rs) | runs the same sweep again at `--jobs N` — M1a T1a.7.5.3, `EIN_JOBS_SWEEP` for the job counts |
| [`ein-cli/tests/summary_properties.rs`](../ein.rs/crates/ein-cli/tests/summary_properties.rs) | the counter identities, over every `solve` cell |

The last four walk the *files* (`ein_corpus::corpus_files`) rather than the
manifest's rows, because their subject is a surface rather than an invocation.
The completeness check is what keeps the two views the same set.

**That difference is what a dropped run does and does not cost**, and it is
worth knowing before reading the `runs` columns below. `corpus_shapes` solves
every corpus *file* three ways and `summary_properties` five, both under an
entering budget, so removing `solve` from an entry's `runs` removes **no
surface**: what it removes is the *process* — the argv the CLI must still
accept, the exit code, the diagnostic on stderr, the `--json-summary` file —
and the fact that the run was **unbudgeted**. On a fixture with no hypothesis
structure, that last one is the only thing a `solve` cell was contributing,
and it contributes it in the form of a blind enumeration nobody reads.

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
slow   = true                      # excluded from the default sweep — see § `slow`
cost_ms = 3459                     # what its runs cost, together, measured
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

**Six groups, and no empty ones.** There were two: `golden` until
[Q-M1a.9](../plans/m1a_rust/open_questions.md#q-m1a9--where-do-goldens-live)
was answered (goldens live in `ein.rs/crates/<crate>/tests/golden/`), and
`generated` until
[S1a.10.4](../plans/m1a_rust/p1a.10_single_implementation/s1a.10.4_utils.md).
`generated` named the throwaway manifest `utils/fuzz_ein.py` wrote to hand a
batch to the parity harness; the rewritten fuzzer drives the `ein` binary
directly and writes no manifest, so the name has no referent. **A corpus entry
is a file the engine is *permanently* exercised over**, and a generated case
lives for milliseconds — which is why the group could never have held one
anyway. A fuzz find that is worth keeping goes to
[`fuzz_findings/`](fuzz_findings/README.md) and, once its expectation
settles, becomes a `regression` entry with a name.

An empty group is a question with a home; both questions are answered, so
neither group is here.

## Dropped runs

Some entries do not declare `solve`, or `solve -e`, or `render lattice`, and
each says so in its `note`. Every reason is about the **puzzle**:

- **A demo that closes no domain.** The `saturation/**` demos exist to show ONE
  rule firing; `features/02_star_in_identifiers` is a lexer demo;
  `features/04_open` demonstrates the `open` macro and
  `features/05_stdlib_domain_elim` the stdlib's elimination rules. None of them
  states a domain, so `solve` on one is not solving the demo — it is
  enumerating everything that could be built out of the demo's objects, to the
  `-m` cap, and calling the result `Contradiction`
  ([Q-M1d.6](../plans/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)).
  `saturate` is the run that asks the demo's question and it costs 3.5 ms.
- **`zebra2-minus-15`** is the honest case: genuinely under-determined at 32
  models, so its exhaustive search is large rather than pathological. Its `-e`
  is [M1d](../plans/m1d_satisfiability/README.md)'s subject and arrives with
  that milestone.
- **`render lattice` is `solve -e` with a DOT writer** (`-m 3` rather than 5 —
  `render.rs::cmd_lattice`), so it inherits the same question and the same
  answer. On `zebra2.ein` it draws **26.8 KB and 210 lines**, which is a view;
  on `square-unique/cul-de-sac` it draws **49.2 MB and 480 934 lines** after
  94 s, which is the enumerator's shadow. Four entries do without it —
  `square-unique/{corner-house,cul-de-sac,terminus}` and `zebra2-minus-15` —
  and **one keeps it**: `features/04_open`, because that fixture's subject *is*
  what an open domain does to a search
  ([Q-M1d.3](../plans/m1d_satisfiability/open_questions.md#q-m1d3--what-closes-a-domain)).
  The corpus pays for exactly one unbounded lattice, and it is the one that
  demonstrates something.

**Do not drop a run merely because it is slow.** The test is whether the run
*asks the fixture's question* — `features/05_stdlib_domain_elim` solves in
3.0 s today and is still not declared, because three seconds of blind
enumeration is not three seconds of coverage; `branching/07_lookahead_off`
costs 0.32 s per run — 0.92 s until T1a.7.2.0 — and keeps both, because on that
fixture the cost **is** the finding. And where a run is dropped the note says which and why: a `runs`
column that shrinks without a reason is how coverage disappears.

**These notes used to blame CPython** — *"outlives a 150 s budget under
CPython, and a run nobody can finish is not coverage"*, on six entries. That
reason was wrong even where its conclusion was right, and
[S1a.9.0](../plans/m1a_rust/p1a.9_release/s1a.9.0_slow_corpus.md) re-priced
every one of them against ein.rs. Three of the excluded runs finish —
`features/05_stdlib_domain_elim`'s two at 3.0 s and `zebra2-minus-15 :: render
lattice` at 27.8 s — and the eight `solve` / `solve -e` cells that do not are
not waiting on a budget at all: at the default `-m 5` they are killed by the
**OOM killer**, at 14 GB and between one and three minutes. An unbounded
hypothesis space is unbounded in memory first, and that would have been true of
any engine.

## `slow`

**An entry is `slow` when its declared runs cost 1 s or more, together**, on
the build and machine
[`corpus_cost.md`](../plans/m1a_rust/p1a.9_release/corpus_cost.md) names.
`cost_ms` records that sum, and two tests hold the flag to it: `ein-corpus`'s
`slow_matches_the_recorded_cost` (exact, arithmetic, never flakes) and
`corpus_cli`'s `the_slow_flag_still_describes_the_sweep` (the wall clock of
the sweep it has just run, at a 4× tolerance).

**Two** entries are slow — `features/01_not_and_absent` and
`features/04_open` — 12 cells and 13.2 s of engine. The default selection is
**629 cells** and 5.3 s of `cargo test`; with `EIN_CORPUS_SLOW=1` the whole 641
take **19.8 s**, where before S1a.9.0 they took **307 s**.

> **Three, until M1a T1a.7.2.0.** `branching/07_lookahead_off`'s seven cells
> cost 2.12 s when S1a.9.0 measured them and cost **754 ms** now: coalescing
> root's layer stack at the search's layer barrier is 2.8× on that entry, and
> the flag stopped being true. Nothing was dropped and nothing was tuned — a
> re-take found it, which is the whole reason `slow` carries a number
> ([scaling.md §6](../plans/m1a_rust/p1a.7_parallelism/scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier)).
> The same pass re-priced the other two: `features/01` 4.06 → 3.46 s, most of
> it T1a.7.1.7's per-worker provenance region, and `features/04` 10.21 →
> 9.74 s.

Three decisions are worth knowing before editing the column:

- **The sum, not the slowest run.** The flag's job is the sweep's budget, and
  the sum is what an entry costs it. The one entry the two rules used to
  disagree about was `branching/07_lookahead_off`, whose two 0.92 s runs were
  under a per-run line and over the sum's — and it is now under both.
- **One second**, because the whole default selection is under three seconds:
  past that an entry stops being part of the sweep and becomes the reason it
  is slow. The measured distribution leaves room on both sides — the slow
  entries cost 3.5 s and 9.7 s, and the most expensive unflagged one 0.75 s —
  so a machine would have to be 3.5× faster or 1.3× slower before the line
  moved. It was 2.1 s / 4.1 s / 10.2 s against 0.38 s until T1a.7.2.0 took the
  2.1 s entry to 0.75 s, and that entry is now the one nearest the line from
  below: the room on the *slow* side grew, the room below it shrank.
- **`cost_ms` is recorded only where something checks it**: on a `slow` entry,
  or on one within 4× of the threshold. Below that the sweep's own tolerance
  swallows it and it would be a number nobody verifies — which is exactly what
  the flag was between 2026-08-17 and S1a.9.0.

Before that stage the flag meant *"one of its runs took 3 s or more under
CPython in the T1a.0.1.1 probe"* and covered 17 entries and 118 cells, four
minutes of sweep. Measured against the engine that ships, **twelve of the
seventeen cost under a second all told** — `zebra2.ein`, the corpus's
flagship, at 16 ms — and two more were flagged only for the 94 and 125 seconds
they spent drawing a commitment lattice for a demo of one rule firing. The
three that stay are slow for reasons their fixtures are about.

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
