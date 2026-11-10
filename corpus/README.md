# corpus/

**The inventory**: every `.ein` file the engine is exercised over, and the
invocations each one is exercised under.

- **`corpus.toml`** — the manifest. Generated once from the tree, maintained by
  hand thereafter. A completeness check fails on any `.ein` under `examples/`,
  `stdlib/` or `tests/` with no entry, so the corpus cannot silently miss a
  file. The third root arrived with M1c
  [S1c.1.4](../docs/history/m1c_external_validation/README.md#s1c14--the-stdlib-corpus)
  and [`tests/README.md`](../tests/README.md) says why a stdlib conformance
  program is not an example.
- **`fuzz_findings/`** — minimised inputs a fuzzer found something on.
- **`wrong_answers.toml`** — the register of known wrong answers
  ([M1e2](../plans/m1e2_correctness/README.md) S1e2.1.1): one row per witness,
  with the engine's answer today and the right one, as data. Its header is the
  format.

## Who reads it

Everything is `cargo test`.

| reader | what it does with the manifest |
|---|---|
| [`ein-cli/tests/corpus_cli.rs`](../ein.rs/crates/ein-cli/tests/corpus_cli.rs) | **the sweep** — runs every entry under every declared run, as a process, holds each cell's exit code to a banked golden, and times them: it is what holds `cost_ms` and `slow` to the wall clock |
| [`ein-corpus/src/manifest.rs`](../ein.rs/crates/ein-corpus/src/manifest.rs) | the completeness check and the manifest's own invariants, the `slow` and `heavy_dump` sets pinned among them |
| [`ein-infer/tests/corpus_cores.rs`](../ein.rs/crates/ein-infer/tests/corpus_cores.rs) | **the class-4 claim** (M1e2 S1e2.2.21): every core a run reports for a dead node on an alive root, the oracle's cube and describe's N, names its commitment and, with root, derives its contradiction; walks the *files* and the register's witnesses, and reads `slow` to leave the zebra family to the nightly (`EIN_CORPUS_SLOW=1`) |
| [`ein-cli/tests/answer_round_trip.rs`](../ein.rs/crates/ein-cli/tests/answer_round_trip.rs) | **the dump's round trip** (M1e2 S1e2.3.8): every entry's `ein solve FILE --dump-answer DIR`, declared or not, each state file reloaded to its state by value; reads `slow` and `heavy_dump` to leave those entries to the nightly (`EIN_CORPUS_SLOW=1`), and holds `heavy_dump` to the state files it counts (M1f S1f.5.13) |
| [`ein-render/tests/corpus_shapes.rs`](../ein.rs/crates/ein-render/tests/corpus_shapes.rs) | digests every observable surface of every corpus *file* (7 462 renderings), which is a superset of what the runs reach |
| [`ein-render/tests/id_order_invariance.rs`](../ein.rs/crates/ein-render/tests/id_order_invariance.rs) | runs the same sweep twice under a permuted id space |
| [`ein-render/tests/jobs_invariance.rs`](../ein.rs/crates/ein-render/tests/jobs_invariance.rs) | runs the same sweep again at `--jobs N` — M1a T1a.7.5.3, `EIN_JOBS_SWEEP` for the job counts |
| [`ein-cli/tests/summary_properties.rs`](../ein.rs/crates/ein-cli/tests/summary_properties.rs) | the counter identities, over every `solve` cell |
| [`ein-cli/tests/wrong_answers.rs`](../ein.rs/crates/ein-cli/tests/wrong_answers.rs) | **the wrong-answer register**, [`wrong_answers.toml`](wrong_answers.toml) (M1e2 S1e2.1.1): one row per witness of every known wrong answer, holding each to the answer it gives today — the wrong one while `open`, the right one once `fixed`. Every witness is a manifest entry or a file under `fuzz_findings/`, which `ein-corpus`'s `register` module checks without a run |

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
runs   = ["solve", "solve --maximal", "saturate", "render rules"]
levers = ["-y"]                    # each makes one more `solve <lever>` run
slow   = true                      # excluded from the default sweep — see § `slow`
cost_ms = 3459                     # what its runs cost, together, measured
heavy_dump = true                  # out of the per-commit round trip — see § `heavy_dump`
note   = "why this entry is interesting, when it is not obvious"
```

A **run name is the `ein` argv with the file position elided**: `"solve
--maximal"` is `ein solve <path> --maximal`, `"render rules"` is `ein render
rules <path>`. Two
substitutions happen in the sweep:

- `{out}` expands to that cell's output directory, so a run can name its own
  artefacts — `"solve --trace {out}/trace.md"`, `"oracle --dump {out}/region"`;
- every `solve` run silently gains `--json-summary {out}/summary.json`.

`runs` is **the invocations this entry is exercised under**, and each one is
a thing that must work.

**Exit 2 means two things, and a run has to say which.** It is the CLI's
usage-error code *and* the code a `-T` abort takes, plus `ein test`'s load
error and nothing-to-check. Since M1e S1e.3.6 `no_cell_crashes` reads the run:
**a `2` is allowed exactly where the run names `-E` / `-T` /
`--max-enterings` / `--max-time`**. Since M1e2 S1e2.1a.5 no cell does: a
budget that stops a description (`--region-budget`, `--region-limit`) is an
answer and exits 0, so `examples/zebra2.ein`'s walk cell `solve -E 1` became
`solve --region-budget 1`, which exits 0 with a `note:`, and `-E` is retired.
`ein oracle` over its `--region-limit` exits 2 too, and is refused, not
budgeted, so no cell declares one (`lattice/06`'s two entries say so).

**`-T` is refused in a cell** — M1e S1e.4.5, `TE-L1`. `-T` counts seconds, so
a `-T` cell's **exit code is a stopwatch** and `corpus_exits.txt` banks it
line by line.
Measured 2026-09-01, the same argv exits 2 at `-T 0.001` and 0 at `-T 60`.
That would be the only place in this repo where a wall clock reaches a banked
artefact — everywhere else the clock is scrubbed before a comparison — so it is
a check rather than a sentence: `ein-corpus`'s
`no_declared_run_budgets_by_wall_clock` fails on a `runs` or `levers` entry
naming `-T` or `--max-time`. The allowance above stays as it is, because the
*CLI* still takes both. A `test` cell on a file that does not load, or that
claims nothing, would exit 2 for a third reason and is **not** in the
allowance: add the cell and the rule together, or the check cannot tell it from
a stale argv.

**`test` is the one run name that can fail on the file's own account.** M1c
[S1c.1.3](../docs/history/m1c_external_validation/README.md#s1c13--ein-test)
added `ein test <path>`, which runs whatever `:expect` the file's queries carry
and exits 1 when one of them is false — so a `test` cell in the exit golden is
banking a claim the *program* makes, not one the engine happens to render. It
is declared on every entry that carries an `:expect` — the three
`examples/features/1{0,1,2}_expect*.ein` fixtures and every program under
[`tests/stdlib/`](../tests/) — and on anything else it would exit 2 with
"nothing to check", which is the right answer and not a useful cell. Adding one
to a file that grows an `:expect` is the growth rule below, applied.

## Groups

| group | what it holds | the sweep expects |
|---|---|---|
| `positive` | `examples/**/*.ein` outside `broken/` and `ein-bugs/` | at least one run answers; catalogued in [`examples/README.md`](../examples/README.md) |
| `stdlib` | the [stdlib](../stdlib/) modules loaded standalone, **and the conformance programs under [`tests/stdlib/`](../tests/) that exercise their rules** — one per rule or tight family, `find tests/stdlib -name '*.ein' \| wc -l` (M1c S1c.1.4; the closed-and-owing pair is M1d S1d.2.2) | as `positive`. The seven module entries exercise the import + macro machinery on their own terms; the `tests/` entries each carry an `:expect`, so their `test` cell is a claim the program makes |
| `parse-negative` | `examples/broken/*.ein` | every run refused, `IRParseError` with `file:line:col` |
| `load-negative` | `examples/broken/load/*.ein` | parse, then fail to load; the exact message is checked in beside each fixture ([README](../examples/broken/load/README.md)) |
| `compile-negative` | `examples/broken/compile/*.ein` | parse and load, then the compiler refuses; `.expected` beside each ([README](../examples/broken/compile/README.md)). **No entry since M1f S1f.5.24**, which moved the last, `eq_in_a_macro_body`, to `broken/load/` (W66): no program reaches a compile refusal. `activator_arity.ein` sits in that directory and is `positive`: its error is unreachable through the engine by design, so the file solves and derives nothing, which is what it pins |
| `regression` | `examples/ein-bugs/*.ein` | **nothing uniform** — see below |

A negative fails whichever way you enter, so what varies is *which entry point
reports it*: the negative groups run `solve`, `saturate` and `render rules` —
three presentations of one error, and three chances to format one of them
differently. With one caveat the sweep measured and this table did not use to
admit: **`render rules` does not load the KB**, so ten of the thirty
load-negatives render their rules and exit 0. That is not a hole, it is where
the pass boundary is; the exit golden records it per cell.

`regression` is the group with no rule, and deliberately. It holds the inputs
that once broke the engine — an uncompilable query goal, a goal binding that
was a JSON number, an `(or …)` whose arms bind different variables — and what
"correct" means for one of them is whatever it does now.
Seven of its ten entries answer on every run, one answers under `saturate` and
is refused under `solve`, two are refused outright — and the exit golden is the
only statement of which.

**A group is a directory, with one deliberate widening.** `tests/stdlib/`'s
entries are `stdlib` rather than a seventh group, because there the group names
a *subject* rather than a failure point: what a `tests/stdlib/` entry is about
is a rule of `stdlib/`, and nothing else in the corpus is. The alternative was
a `stdlib-conformance` group holding the tests and nothing else, which is a
vocabulary entry that means "the directory it is named after".

**Six groups, and no empty ones.** There were two: `golden` until
[Q-M1a.9](../docs/history/m1a_rust/open_questions.md#q-m1a9--where-do-goldens-live)
was answered (goldens live in `ein.rs/crates/<crate>/tests/golden/`), and
`generated` until
[S1a.10.4](../docs/history/m1a_rust/README.md#s1a104--utils-and-the-fuzzer).
`generated` named a throwaway manifest of fuzzer cases; `utils/fuzz_engine.py`
drives the `ein` binary directly and writes no manifest, so the name has no
referent. **A corpus entry
is a file the engine is *permanently* exercised over**, and a generated case
lives for milliseconds — which is why the group could never have held one
anyway. A fuzz find that is worth keeping goes to
[`fuzz_findings/`](fuzz_findings/README.md) and, once its expectation
settles, becomes a `regression` entry with a name.

An empty group is a question with a home; both questions are answered, so
neither group is here.

## Dropped runs

Some entries do not declare `solve`, or `render lattice`, and each says so in
its `note`. Every reason is about the **puzzle**. Until M1e2 S1e2.1a.5 the
list was about the walk's `solve` and `solve -e`, and the reasons below are
the ones those notes give:

- **A demo that closes no domain.** The `saturation/**` demos exist to show ONE
  rule firing; `features/02_star_in_identifiers` is a lexer demo;
  `features/04_open` demonstrates the `unknown` macro (named `open` when the file was) and
  `features/05_stdlib_domain_elim` the stdlib's elimination rules. None of them
  states a domain, so `solve` on one is not solving the demo — it is
  enumerating everything that could be built out of the demo's objects, to the
  `-m` cap, and calling the result `Contradiction`
  ([Q-M1d.6](../docs/history/m1d_satisfiability/open_questions.md#q-m1d6--may-contradiction-be-said-with-exhausted--false)).
  `saturate` is the run that asks the demo's question and it costs 3.5 ms.
- **`zebra2-minus-15`** is the honest case: genuinely under-determined at 32
  models, so its exhaustive search is large rather than pathological. Its `-e`
  is [M1d](../docs/history/m1d_satisfiability/README.md)'s subject and arrives with
  that milestone.
- **`render lattice` is `solve -e` with a DOT writer** (`-m 3` rather than 5 —
  `render.rs::cmd_lattice`), so it inherits the same question and the same
  answer. On `zebra2.ein` it draws **26.8 KB and 210 lines**, which is a view;
  on `square-unique/cul-de-sac` it draws **49.2 MB and 480 934 lines** after
  94 s, which is the enumerator's shadow. Four entries do without it —
  `square-unique/{corner-house,cul-de-sac,terminus}` and `zebra2-minus-15` —
  and **one keeps it**: `features/04_open`, because that fixture's subject *is*
  what an open domain does to a search
  ([Q-M1d.3](../docs/history/m1d_satisfiability/open_questions.md#q-m1d3--what-closes-a-domain)).
  The corpus pays for exactly one unbounded lattice, and it is the one that
  demonstrates something.

**Do not drop a run merely because it is slow.** The test is whether the run
*asks the fixture's question* — `features/05_stdlib_domain_elim` solves in
3.0 s today and is still not declared, because three seconds of blind
enumeration is not three seconds of coverage; `branching/07_lookahead_off`
costs 0.32 s per run — 0.92 s until T1a.7.2.0 — and keeps both, because on that
fixture the cost **is** the finding. And where a run is dropped the note says which and why: a `runs`
column that shrinks without a reason is how coverage disappears.

[S1a.9.0](../docs/history/m1a_rust/README.md#s1a90--the-slow-corpus-re-priced) priced
every excluded run against the engine. Three of them finish —
`features/05_stdlib_domain_elim`'s two at 3.0 s and `zebra2-minus-15 :: render
lattice` at 27.8 s — and the eight `solve` / `solve -e` cells that do not are
not waiting on a budget at all: at the default `-m 5` they are killed by the
**OOM killer**, at 14 GB and between one and three minutes. An unbounded
hypothesis space is unbounded in memory first.

## `slow`

**An entry is `slow` when its declared runs cost 1 s or more, together**, on
the build and machine
[`corpus_cost.md`](../docs/history/m1a_rust/measurements/corpus_cost.md) names.
`cost_ms` records that sum, and two tests hold the flag to it: `ein-corpus`'s
`slow_matches_the_recorded_cost` (exact, arithmetic, never flakes) and
`corpus_cli`'s `the_slow_flag_still_describes_the_sweep` (the wall clock of
the sweep it has just run, at a 4× tolerance).

**Which entries are slow is a test's to say**, and this page does not copy its
list: `ein-corpus`'s `the_slow_set_is_exactly_these_fourteen` names them, in
path order, and fails per commit when the set moves (M1e S1e.3.6; its name has
moved with the set), and `grep -c '^slow *= *true' corpus/corpus.toml` counts
them. A re-take moves the flag both ways: `features/04_open` lost it at M1e2
S1e2.1a.5's switch, when no cell ran the walk any more, and the six `square-*`
gained it at S1e2.1c.2. What each costs is
[`corpus_cost.md`](../docs/history/m1a_rust/measurements/corpus_cost.md)'s, whose
§ 10 is the latest re-take (2026-11-03).

The **cell count** is `wc -l ein.rs/crates/ein-cli/tests/golden/corpus_exits.txt`:
the exit golden is one line per cell of the full selection, so it is the number
and it cannot drift from the sweep that writes it. The default selection is
that minus the slow entries' cells. What the sweep costs is a
measurement — **5.3 s** of `cargo test` by default and **22.2 s** with
`EIN_CORPUS_SLOW=1`, where before S1a.9.0 they took **307 s** (2026-08-22, and
`utils/corpus_cost.py` re-takes it).

> It was 629 / 641 cells at S1a.9.0 and the wall clock has not moved: M1c
> S1c.1.2 added five fixtures and S1c.1.4 added 45 with 225 cells between
> them, and the default sweep still finishes in the same 5.3 s, because a
> conformance program is a dozen facts and the cost of a cell there is the
> process, not the engine.

> **Three, until M1a T1a.7.2.0.** `branching/07_lookahead_off`'s seven cells
> cost 2.12 s when S1a.9.0 measured them and **754 ms** after it: coalescing
> root's layer stack at the search's layer barrier is 2.8× on that entry, and
> the flag stopped being true. Nothing was dropped and nothing was tuned — a
> re-take found it, which is the whole reason `slow` carries a number
> ([scaling.md §6](../docs/history/m1a_rust/measurements/scaling.md#6-t1a720--the-layer-stack-coalesced-at-the-barrier)).
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
  is slow. When the line was drawn the measured distribution left room on both
  sides (M1a T1a.7.2.0, 2026-08-22, `corpus_cost.md` § 7): the two slow
  entries cost 3.5 s and 9.7 s, and the most expensive unflagged one 0.75 s,
  so a machine would have had to be 3.5× faster or 1.3× slower before the
  line moved. It was 2.1 s / 4.1 s / 10.2 s against 0.38 s until T1a.7.2.0
  took the 2.1 s entry to 0.75 s. The distribution since is each re-take's in
  `corpus_cost.md`, which moved entries across the line both ways.
- **`cost_ms` is recorded only where something checks it**: on a `slow` entry,
  or on one within 4× of the threshold. Below that the sweep's own tolerance
  swallows it and it would be a number nobody verifies — which is exactly what
  the flag was between 2026-08-17 and S1a.9.0.

## `heavy_dump`

**An entry is `heavy_dump` when its describe dump writes 40 000 state files or more** (M1f S1f.5.13, the user's
ruling of 2026-11-08 on P-16, *"move to slow tier"*). The dump is `ein solve FILE --dump-answer DIR`, which no entry
declares and [`ein-cli/tests/answer_round_trip.rs`](../ein.rs/crates/ein-cli/tests/answer_round_trip.rs) runs on every
entry, reloading each state file to its state by value; the line is `ein_corpus::manifest::HEAVY_DUMP_FILES`.

- **Per commit** the round trip leaves out every entry that is `slow` or `heavy_dump`, and fails on an unflagged one
  that writes 40 000 or more. **Under `EIN_CORPUS_SLOW=1`**, which the nightly's `deep-corpus` job sets, as
  `./run_tests.sh --slow`, `--nightly` and `--dev-host` do, it takes every entry and fails as well on a flagged one that writes
  fewer. `ein-corpus`'s `the_heavy_dump_set_is_exactly_these_five` pins the set per commit, and `nightly_steps.rs`
  that the nightly sets the flag.
- **Not `slow`**, which is a claim about the declared runs: the dump is none of them, and the four entries the ruling
  moved declare runs that cost milliseconds, so `slow` on them would be the false claim
  `slow_matches_the_recorded_cost` fails on.
- **A count, not a time**, so it holds on any machine and moves only when a dump does. The line sits in the gap the
  distribution of 2026-11-09 leaves above the six `slow` `square-*` (`EIN_ROUND_TRIP_LIST=1 EIN_CORPUS_SLOW=1 cargo
  test … -p ein-cli --release --test answer_round_trip -- --nocapture`): five entries write 64 514 to 65 214 state
  files, then come 23 390 (the six), 11 339 (`type-exclusivity/pets`) and 4 096 or fewer.

**Five entries are flagged**: `square-unique/corner-house`, `cul-de-sac` and `terminus`, `features/04_open`, and
`features/01_not_and_absent`, which is `slow` too and carries the flag because the slow tier holds it on every entry.
Per commit the round trip took 8.6 s where it took 55 s, and the slow tier takes 121 s in release (2026-11-09,
[S1f.5.13's T2 record](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.13_large_dumps_to_the_slow_tier.md#t2-record-2026-11-09)).

## Tiers

**Three tiers run the corpus's tests** (M1f S1f.5.27, the user's ruling of 2026-11-10: *"reduce nightly job, remove
long running tests, we will eventually run them on dev host, retain critical tests in CI"*): the per-commit gate
(`./run_tests.sh`, `per-commit.yml`), the CI nightly (`./run_tests.sh --nightly`, `nightly.yml`) and the dev host
(`./run_tests.sh --dev-host`), which nothing schedules. The nightly keeps a step that guards an answer or a contract no
per-commit step guards, at a budget that fits in 5 min on the runner; a step past that, or one that measures rather
than checks, is the dev host's, which also runs every nightly step at full budget
([Q-M1f.102](../plans/m1f_hypothesis_and_documentation/open_questions.md#q-m1f102--what-the-ci-nightly-keeps-and-what-the-dev-host-runs)).
`ein-cli/tests/gate_steps.rs` holds the gate's list to its workflow, and `nightly_steps.rs` the nightly's to its
workflow and the dev host's to the nightly's.

| test or script | per commit | CI nightly | dev host |
|---|---|---|---|
| `corpus_cli` | every entry but the `slow` ones | the `slow` ones too (`EIN_CORPUS_SLOW=1`, `--release`) | as the nightly |
| `answer_round_trip` | every entry but the `slow` and `heavy_dump` ones | every entry | as the nightly |
| `lever_invariance` | every declared lever cell but the `slow` ones | the `slow` cells too | as the nightly |
| `corpus_cores` | every entry but the `slow` ones | the `slow` ones too | as the nightly |
| `answer_dump` | every entry | the same, in release | as the nightly |
| `region`, `obligation_rung` | the fast halves | the slow halves too, since S1f.5.27 | as the nightly |
| `route_invariance` | every monotone program both routes describe at the default budget | — | the zebra family and the six `square-*` at `--region-budget` 2²⁵ too |
| `id_order_invariance` | one permutation of the id space | — | eight |
| `fuzz_properties` | 2 000 inputs | 20 000 a seed, two seeds | 1 000 000 a seed |
| `utils/stdlib_mutants.py --check` | — | yes | yes |
| `utils/stdlib_census.py --check` | — (`stdlib_coverage` is the gate) | — | yes |
| `utils/review_bundle.py` | `--smoke` | — | the whole bundle, and its link check |
| `cargo bench --bench engine -- --test` | yes | yes | yes |

What each step took, on the runner and here, is in
[S1f.5.27's records](../plans/m1f_hypothesis_and_documentation/p1f.5_the_no_owner_items/s1f.5.27_the_nightly_reduced.md#t1-record-2026-11-10).

## Levers

`levers` names the CLI levers a program is run under, each one more `solve
<lever>` run: `-L` (`enable-pre-branch-lookahead` off), `-K`
(`enable-lookahead-kill-cache` off) and `-y` (`lattice-sanity-check` on).
**Twelve entries declare 30 cells** (2026-11-01). `-o score-sum` was the fourth until M1e2
S1e2.1a.5's switch retired it with the walk's order levers. The other flags a
program can set are reachable only through its own `(config …)` block, so a
process sweep cannot flip them from outside
([Q-M1a.16](../docs/history/m1a_rust/open_questions.md#q-m1a16--how-does-the-corpus-drive-the-lever-matrix)).

**Since the switch no route reads a lever**, and a cell is a claim that it
changes nothing printed: C4 of
[`solve_interface.md`](../plans/m1e2_correctness/p1e2.1a_the_solution_region/solve_interface.md#1-the-contract),
and the user's config principle of 2026-10-15
([`configuration.md` § 2.5](../docs/kernel/configuration.md#25-the-config-principle-read-against-every-row)).
`corpus_cli.rs` runs each cell for its exit code. Since M1e2 S1e2.1a.6
**`ein-cli/tests/lever_invariance.rs` holds what it prints**: every declared
cell under each of `ein solve`'s fifteen questions, stdout, stderr, the exit
code and the summary, against the same question at the default, and every
flag a `(config …)` can set on every program whose R is known. The zebra2
pair's cells are `slow` and run in the nightly, under `EIN_CORPUS_SLOW=1`.
A lever that changes an answer is a register row, never a weaker mode.

**Before the switch the levers were the walk's**, and `-L` was *a strictly
weaker debug mode*: `complete()` approximated maximality more weakly with the
lookahead off, so its cells checked a weaker mode rather than giving a second
opinion
([`branching/15`+`16`](../examples/branching/15_lookahead_two_step_on.ein)
showed both sides wrong), and `-K` removed writes the search made into the KB
it recorded
([Q-M1e.7](../docs/history/m1e_review_processing/open_questions.md#q-m1e7--the-read-out-prints-the-solution-kb-and-calls-it-a-model)).
Under R the register filed both: `-L`'s weaker mode as W1's `-L` rows, and
the default's writes as W9, where `-K` was the run that agreed with R.

## Growth rule

Any defect found outside the corpus becomes a corpus entry in the same commit
that fixes it. The corpus only grows.
