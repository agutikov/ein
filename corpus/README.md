# corpus/

**The inventory**: every `.ein` file the engine is exercised over, and the
invocations each one is exercised under.

- **`corpus.toml`** — the manifest, maintained by hand. A completeness check
  fails on any `.ein` under `examples/`, `stdlib/` or `tests/` with no entry,
  so the corpus cannot silently miss a file;
  [`tests/README.md`](../tests/README.md) says why a stdlib conformance
  program is not an example.
- **`fuzz_findings/`** — minimised inputs a fuzzer found something on.
- **`regressions.toml`** — the regressions: one row per program and run that
  was once answered wrong, with the engine's answer today and the right one,
  as data, each held to the right one. Its header is the format.

## Who reads it

Everything is `cargo test`.

| reader | what it does with the manifest |
|---|---|
| [`ein-cli/tests/corpus_cli.rs`](../ein.rs/crates/ein-cli/tests/corpus_cli.rs) | **the sweep** — runs every entry under every declared run, as a process, holds each cell's exit code to a banked golden, and times them: it is what holds `cost_ms` and `slow` to the wall clock |
| [`ein-corpus/src/manifest.rs`](../ein.rs/crates/ein-corpus/src/manifest.rs) | the completeness check and the manifest's own invariants, the `slow` and `heavy_dump` sets pinned among them |
| [`ein-infer/tests/corpus_cores.rs`](../ein.rs/crates/ein-infer/tests/corpus_cores.rs) | **every core names its cause**: every core a run reports for a dead node on an alive root, the oracle's cube and describe's N, names its commitment and, with root, derives its contradiction; walks the *files* and the regressions' witnesses, and reads `slow` to leave the zebra family to the slow half |
| [`ein-cli/tests/answer_round_trip.rs`](../ein.rs/crates/ein-cli/tests/answer_round_trip.rs) | **the dump's round trip**: every entry's `ein solve FILE --dump-answer DIR`, declared or not, each state file loaded as the state the run saved, and saved and loaded again in both directions; reads `slow` and `heavy_dump` to leave those entries to the slow half, and holds `heavy_dump` to the state files it counts |
| [`ein-render/tests/id_order_invariance.rs`](../ein.rs/crates/ein-render/tests/id_order_invariance.rs) | digests every observable surface of a corpus *file* against `golden/corpus_shapes.md5`, a superset of what the runs reach, and runs it again under a permuted id space |
| [`ein-render/tests/renaming_invariance.rs`](../ein.rs/crates/ein-render/tests/renaming_invariance.rs) | **the kernel reads no theory's name**: runs every corpus *file* again with every name that is not the kernel's renamed, in it and in every module it imports, and holds the answer, the claims' verdicts and every drawing to the file's own, apart from the names |
| [`ein-render/tests/jobs_invariance.rs`](../ein.rs/crates/ein-render/tests/jobs_invariance.rs) | runs the ops a job count reaches again at `--jobs N`, each held to the golden's digest of its `--jobs 1` rendering and computed again only where it differs; `EIN_JOBS_SWEEP` names the job counts |
| [`ein-cli/tests/summary_properties.rs`](../ein.rs/crates/ein-cli/tests/summary_properties.rs) | the counter identities, over every `solve` cell |
| [`ein-cli/tests/regressions.rs`](../ein.rs/crates/ein-cli/tests/regressions.rs) | **the regressions**, [`regressions.toml`](regressions.toml): one row per program and run once answered wrong, each run through the binary and held to the right answer it records, and each witness's region recomputed with the library. Every witness is a manifest entry or a file under `fuzz_findings/`, and every row's recorded answer is its right one, which `ein-corpus`'s `regressions` module checks without a run |

`corpus_cores`, `id_order_invariance`, `jobs_invariance` and `renaming_invariance` walk the *files* (`ein_corpus::corpus_files`) rather than the
manifest's rows, because their subject is a surface rather than an invocation.
The completeness check is what keeps the two views the same set.

**Every reader above that sweeps the corpus runs on the worker pool**
(`ein_corpus::pool`): its units — a (file, op) pair, a file, a cell — run
on `EIN_TEST_JOBS` workers, default 16, and are read back in their order, so no
verdict and no failure's text depends on the count. A sweep whose cells it
times, `corpus_cli`, times each beside the cells the other workers run. The
sweeps over (file, op) pairs load each file once and run every op on a copy
of the load (`corpus_ops::Loads`).

**Which files, by default and in the slow half.** By default the golden and
`dot_wellformed` read every file outside `tests/`, whose programs are each one
stdlib rule's claim and are held by their claims; the two determinism sweeps
read those of them that **search**, whose describe saturates more than root
(`corpus_ops::searches`, computed, never listed); `route_invariance`, every
file but the `slow` entries. `EIN_CORPUS_SLOW=1` reads every file in each. A
long sweep can be split, `EIN_TEST_SHARD=k/n`: `ein_corpus::shard` deals every
file to exactly one shard.

**That difference is what a dropped run does and does not cost**, and it is
worth knowing before reading the `runs` columns below. The golden digests
every rendering of every corpus *file*, and `summary_properties` asks each five
questions under a small budget, so removing `solve` from an entry's `runs`
removes **no surface**: what it removes is the *process* — the argv the CLI
must still accept, the exit code, the diagnostic on stderr, the
`--json-summary` file — and the run at the default budget. On a fixture with no
hypothesis structure, that last one is a blind enumeration nobody reads.

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
runs   = ["solve", "solve --maximal", "saturate", "render rules"]
levers = ["-y"]                    # each makes one more `solve <lever>` run
slow   = true                      # excluded from the default sweep — see § `slow`
cost_ms = 3459                     # what its runs cost, together, measured
heavy_dump = true                  # out of the default round trip — see § `heavy_dump`
note   = "why this entry is interesting, when it is not obvious"
```

A **run name is the `ein` argv with the file position elided**: `"solve
--maximal"` is `ein solve <path> --maximal`, `"render rules"` is `ein render
rules <path>`. Two substitutions happen in the sweep:

- `{out}` expands to that cell's output directory, so a run can name its own
  artefacts — `"solve --trace {out}/trace.md"`, `"oracle --dump {out}/region"`;
- every `solve` run silently gains `--json-summary {out}/summary.json`.

`runs` is **the invocations this entry is exercised under**, and each one is
a thing that must work.

**Exit 2 means two things, and a run has to say which.** It is the CLI's
usage-error code *and* the code a `-T` abort takes, plus `ein test`'s load
error and nothing-to-check. `no_cell_crashes` reads the run: **a `2` is
allowed exactly where the run names `-E` / `-T` / `--max-enterings` /
`--max-time`**, and no cell does. A budget that stops a description
(`--region-budget`, `--region-limit`) is an answer and exits 0, so
`examples/zebra2.ein`'s cell `solve --region-budget 1` exits 0 with a
`note:`, and `-E` is refused by the CLI. `ein oracle` over its
`--region-limit` exits 2 too, and is refused, not budgeted, so no cell
declares one (`lattice/06`'s two entries say so).

**`-T` is refused in a cell.** `-T` counts seconds, so a `-T` cell's **exit
code would be a stopwatch** and `corpus_exits.txt` would bank it line by line.
That would be the only place in this repo where a wall clock reaches a banked
artefact — everywhere else the clock is scrubbed before a comparison — so it
is a check rather than a sentence: `ein-corpus`'s
`no_declared_run_budgets_by_wall_clock` fails on a `runs` or `levers` entry
naming `-T` or `--max-time`. The allowance above stays as it is, because the
*CLI* still takes `-T`. A `test` cell on a file that does not load, or that
claims nothing, would exit 2 for a third reason and is **not** in the
allowance: add the cell and the rule together, or the check cannot tell it
from a stale argv.

**`test` is the one run name that can fail on the file's own account.**
[`ein test <path>`](../docs/ein/user/04-reference/05-cli.md#23-ein-test) checks
whatever `:expect` the file carries and exits 1 when one of them is refuted, so
a `test` cell in the exit golden is banking a claim the *program* makes, not
one the engine happens to render. It is declared on every entry that carries
an `:expect` and that the static check does not flag — the
`examples/features/1{0,1,2,3}_*.ein` fixtures among them, and every program
under [`tests/stdlib/`](../tests/) — and on anything else it would exit 2 with
"nothing to check", which is the right answer and not a useful cell. **A
flagged program's claim is checked by no run**: nothing is claimed of what such
a program answers
([Q-M1f.564](../docs/decisions/answer/Q-M1f.564-a-non-monotone-program-loads-with-a-warning.md)),
so a flagged entry declares no `test` run, and one whose program states an
`:expect` declares only runs that check no claim (`saturate`, a `render` view,
`solve --count`); the manifest's `a_flagged_program_declares_no_run_that_checks_a_claim`
holds it, the flag computed from each program. Adding one to a file that grows an `:expect` is the growth rule
below, applied.

## Groups

| group | what it holds | the sweep expects |
|---|---|---|
| `positive` | `examples/**/*.ein` outside `broken/` and `ein-bugs/`, and the programs under `tests/` outside `tests/stdlib/` | at least one run answers; catalogued in [`examples/README.md`](../examples/README.md) and [`tests/README.md`](../tests/README.md) |
| `stdlib` | the [stdlib](../stdlib/) modules loaded standalone, **and the conformance programs under [`tests/stdlib/`](../tests/) that exercise their rules** — one per rule or tight family, `find tests/stdlib -name '*.ein' \| wc -l` | as `positive`. The module entries exercise the import + macro machinery on their own terms; the `tests/` entries each carry an `:expect`, so their `test` cell is a claim the program makes |
| `parse-negative` | `examples/broken/*.ein` | every run refused, with `file:line:col` |
| `load-negative` | `examples/broken/load/*.ein` | parse, then fail to load; the exact message is checked in beside each fixture ([README](../examples/broken/load/README.md)) |
| `compile-negative` | `examples/broken/compile/*.ein` | parse and load, then the compiler refuses; `.expected` beside each ([README](../examples/broken/compile/README.md)). **It has no entry**: no program reaches a compile refusal, since the parser and the loader refuse every shape that would. `activator_arity.ein` sits in that directory and is `positive`: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
| `regression` | `examples/ein-bugs/*.ein` | **nothing uniform** — see below |

A negative fails whichever way you enter, so what varies is *which entry point
reports it*: the negative groups run `solve`, `saturate` and `render rules` —
three presentations of one error, and three chances to format one of them
differently. With one caveat: **`render rules` does not load the KB**, so a
load-negative can render its rules and exit 0. That is not a hole, it is where
the pass boundary is; the exit golden records it per cell.

`regression` is the group with no rule, and deliberately. It holds the inputs
that once broke the engine — an uncompilable query goal, a goal binding that
was a JSON number, an `(or …)` whose arms bind different variables — and what
"correct" means for one of them is whatever it does now. Some of its entries
answer on every run, some are refused, and some do both — and the exit
golden is the only statement of which (`negatives_are_grouped_by_where_they_fail`
holds the group to the directory).

**A group is a directory, with one deliberate widening.** `tests/stdlib/`'s
entries are `stdlib` rather than a group of their own, because there the group
names a *subject* rather than a failure point: what a `tests/stdlib/` entry is
about is a rule of `stdlib/`, and nothing else in the corpus is. The
alternative was a `stdlib-conformance` group holding the tests and nothing
else, which is a vocabulary entry that means "the directory it is named
after".

**No empty group.** Goldens live beside the crate they pin, in
`ein.rs/crates/<crate>/tests/golden/`
([Q-M1a.9](../docs/decisions/corpus/Q-M1a.9-a-golden-lives-beside-the-crate-it-pins.md)),
not in the corpus. **A corpus entry is a file the engine is *permanently*
exercised over**, and a generated fuzz case lives for milliseconds, so it
never has an entry: a fuzz find that is worth keeping goes to
[`fuzz_findings/`](fuzz_findings/README.md) and, once its expectation
settles, becomes a `regression` entry with a name.

## Dropped runs

Some entries do not declare `solve`, or `render lattice`, and each says why in
its `note`. Every reason is about the **puzzle**:

- **A demo that states no domain.** The `saturation/**` demos exist to show ONE
  rule firing; `features/02_star_in_identifiers` is a lexer demo;
  `features/04_open` demonstrates the `unknown` macro and
  `features/05_stdlib_domain_elim` the stdlib's elimination rules. None of them
  bounds what may be guessed, so `solve` on one is not solving the demo: it is
  enumerating every fact that could be built out of the demo's atoms.
  `saturate` is the run that asks the demo's question.
- **The `syntax/` probes** exist to be drawn: each is one shape the renderers
  must handle, and its runs are the views
  ([`examples/README.md`](../examples/README.md)).

**Do not drop a run merely because it is slow.** The test is whether the run
*asks the fixture's question*: a run that does stays, whatever it costs, and
a slow entry is flagged `slow` rather than trimmed. Where a run is dropped the
note says which and why: a `runs` column that shrinks without a reason is how
coverage disappears.

## `slow`

**An entry is `slow` when its declared runs cost 1 s or more, together**
(`ein_corpus::manifest::SLOW_MS`). `cost_ms` records that sum, measured on a
release build, and two tests hold the flag to it: `ein-corpus`'s
`slow_matches_the_recorded_cost` (exact, arithmetic, never flakes) and
`corpus_cli`'s `the_slow_flag_still_describes_the_sweep` (the wall clock of
the sweep it has just run, within a factor of the threshold; the recorded
`cost_ms` is held within the same factor under `EIN_CORPUS_SLOW=1`).

**Which entries are slow is a test's to say**, and this page does not copy its
list: `ein-corpus`'s `the_slow_set_is_exactly_these_sixteen` names them, in
path order, and fails when the set moves, and
`grep -c '^slow *= *true' corpus/corpus.toml` counts them. A re-take of the
costs can move the flag both ways, which is what a measured flag is for.

The **cell count** is `wc -l ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`:
the exit golden is one line per cell of the full selection, so it is the number
and it cannot drift from the sweep that writes it. The default selection is
that minus the slow entries' cells.

Three decisions are worth knowing before editing the column:

- **The sum, not the slowest run.** The flag's job is the sweep's budget, and
  the sum is what an entry costs it.
- **One second**, so that an entry past it is the reason the default sweep is
  slow, not a part of it.
- **`cost_ms` is recorded only where something checks it**: on a `slow` entry,
  or on one within the tolerance factor of the threshold. Below that the
  sweep's own tolerance swallows it and it would be a number nobody verifies.

## `heavy_dump`

**An entry is `heavy_dump` when its describe dump writes 40 000 state files or
more** (`ein_corpus::manifest::HEAVY_DUMP_FILES`). The dump is
`ein solve FILE --dump-answer DIR`, which no entry declares and
[`ein-cli/tests/answer_round_trip.rs`](../ein.rs/crates/ein-cli/tests/answer_round_trip.rs)
runs on every entry, each state file loaded as the state the run saved and held to the two invariances, save → load
→ save byte for byte and load → save → load dump for dump.

- **By default** the round trip leaves out every entry that is `slow` or
  `heavy_dump`, and fails on an unflagged one that writes 40 000 or more.
  **Under `EIN_CORPUS_SLOW=1`** it takes every entry and fails as well on a
  flagged one that writes fewer. `ein-corpus`'s
  `the_heavy_dump_set_is_exactly_these_five` pins the set.
- **Not `slow`**, which is a claim about the declared runs: the dump is none of
  them, and most flagged entries declare runs that cost milliseconds, so `slow`
  on them would be the false claim `slow_matches_the_recorded_cost` fails on.
- **A count, not a time**, so it holds on any machine and moves only when a
  dump does. The line sits in a gap of the distribution, which
  `EIN_ROUND_TRIP_LIST=1 EIN_CORPUS_SLOW=1 cargo test … -p ein-cli --test
  answer_round_trip -- --nocapture` prints, each program with its count.

**Five entries are flagged**: `square-unique/corner-house`, `cul-de-sac` and
`terminus`, `features/04_open`, and `features/01_not_and_absent`, which is
`slow` too and carries the flag because the slow half holds it on every entry.

## The slow half

Each corpus test has a default half and a slow half, `EIN_CORPUS_SLOW=1`, which
adds what the default leaves out. The tests are built to run under
`cargo test --profile gate` ([`ein.rs/Cargo.toml`](../ein.rs/Cargo.toml)):
release codegen with the debug assertions and overflow checks on.

| test | by default | under `EIN_CORPUS_SLOW=1` |
|---|---|---|
| `corpus_cli` | every entry but the `slow` ones | the `slow` ones too |
| `answer_round_trip` | every entry but the `slow` and `heavy_dump` ones | every entry |
| `lever_invariance` | every declared lever cell but the `slow` ones | the `slow` cells too |
| `corpus_cores` | every entry but the `slow` ones | the `slow` ones too |
| `answer_dump` | every run it records but the `heavy_dump` entries' | the `heavy_dump` entries' too |
| `region`, `obligation_rung` | the fast halves | the slow halves too |
| `pair_layer` | every entry but the `slow` ones, and three named instances | the `slow` ones too |
| `theory_share` | every entry but the `slow` ones, and the instances it names | the same: it has no slow half |
| `cover` | every entry but the `slow` ones, and the instances it names | the same: it has no slow half |
| `structure_invariance` | every program that searches but the `slow` ones, and three named instances | the `slow` ones too |
| `route_invariance` | every monotone program both routes describe at the default budget, but the `slow` entries | the `slow` ones too; no test runs past the default region budget |
| `id_order_invariance` | one permutation of the id space; `EIN_ID_SEEDS=N` takes N | the same, over every file |
| `renaming_invariance` | every file outside `tests/` | every file |
| `fuzz_properties` | 2 000 inputs; `EIN_FUZZ_ITERS` and `EIN_FUZZ_SEED` set the budget and the stream | the same |

## Levers

`levers` names the CLI levers a program is run under, each one more `solve
<lever>` run. The one a manifest declares is `-y` (`lattice-sanity-check`
on). The other flags a program can set are reachable only through its own
`(config …)` block, so a process sweep cannot flip them from outside.

**No route reads a lever**, and a cell is a claim that it changes nothing
printed: C4 of [`05-cli.md`](../docs/ein/user/04-reference/05-cli.md#1-the-contract),
and the config principle
([`06-config.md` § 2.5](../docs/ein/user/04-reference/06-config.md#25-the-config-principle-read-against-every-row),
[Q-M1e2.30](../docs/decisions/answer/Q-M1e2.30-no-configuration-option-lever-or-flag-changes-r.md)).
`corpus_cli.rs` runs each cell for its exit code, and
**`ein-cli/tests/lever_invariance.rs` holds what it prints**: every declared
cell under each of `ein solve`'s questions, stdout, stderr, the exit code and
the summary, against the same question at the default, and every flag a
`(config …)` can set on every program whose R is known. The cells of
`zebra2.ein` and `zebra2-obligations.ein` are `slow`, and run under
`EIN_CORPUS_SLOW=1`. A lever that changes an
answer is a regression, never a weaker mode.

## Growth rule

Any defect found outside the corpus becomes a corpus entry in the same commit
that fixes it. The corpus only grows.
