# S1a.10.4 — `utils/`, re-aimed at one engine

**Phase:** P1a.10 (One implementation)
**Estimate:** 2 days
**Depends on:** [S1a.10.3](s1a.10.3_corpus_without_an_oracle.md)

## Context

**19 of the ~30 scripts in `utils/` name `ein.py`, `pypy`, a venv or
`PYTHONPATH`**, and they do it for three different reasons:

- **comparison** — the script's whole point is two engines
  (`py_oracle.py`, `ir_oracle.py`, `bench_baseline.py`'s Python column,
  `e2e_baseline.py`'s `--impl`, `mutant_ein.py`, `fuzz_ein.py`'s
  differential arm);
- **measurement of ein.py** — scripts that exist to explain the *Python*
  engine (`profile_solve.py`, `count_work.py`, `measure_match_skips.py`,
  `measure_redundant_firings.py`, `symmetric_bench.py`, `fork_split.py`);
- **incidental** — it drives an engine and ein.py is simply the one it
  learned to call (`render_examples.sh`, `zebra2_trace.sh`,
  `feature_matrix.py`, `stdlib_manifest.py`, `find_dead_defs.py`,
  `check_hashmap_iteration.py`, `relation_algebra_examples.py`).

Only the third class is a mechanical edit. The first two need a decision each,
and "delete it" is a legitimate answer for most of the second.

## Acceptance

- Every script either runs against ein.rs or is gone; none is left importing a
  module that no longer exists.
- The **M1a measurement set** still works, because
  [baseline.md](../p1a.6_performance/baseline.md) and
  [scaling.md](../p1a.7_parallelism/scaling.md) are denominated in it:
  `bench_env.sh`, `e2e_baseline.py`, `profile_ein_rs.py`, `criterion_table.py`.
  Their **Python columns become historical constants** in those documents, not
  live measurements — and the documents say which numbers can still be
  re-measured and which are frozen.
- `CLAUDE.md`'s `utils/` description matches what is there.
- A deleted script's *result* survives where the result was the point:
  `feature_matrix_results.json` is the lever matrix's record and outlives its
  runner if the runner goes.

## Tasks

### Task T1a.10.4.1 — The comparison scripts

`fork_delta_verify.py` ([D3](../divergences.md)'s fixture) and
`spec_audit.py` ([S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md)'s
instrument) are **already single-engine** — each runs one binary twice under
different switches — so they survive untouched and are the model for the rest.
`py_oracle.py` / `ir_oracle.py` / `mutant_ein.py` are two-engine by
construction and go with the oracle.

> **What S1a.10.3 already did here.** `mutant_ein.py`'s *claim* — that the D3
> event cut still catches a dropped productive firing — has a successor,
> `ein-infer/tests/event_cut_control.rs`, so the script can be deleted with
> nothing to bank; its header says so. `fork_delta_verify.py` and
> `spec_audit.py` were re-pointed at `corpus/corpus.toml` and widened to
> include the `regression` group, which is the same file set they had before
> the regrouping. **`fuzz_ein.py` is dead right now** — its differ was
> `ein-conformance` — and its docstring and its startup message both say so and
> name T1a.10.4.2. That is the one script in the tree that does not run.

### Task T1a.10.4.2 — The fuzzer

`fuzz_ein.py` keeps its generator and its self-checkable properties and loses
its differential arm — the decision is
[S1a.10.1](s1a.10.1_bank_the_oracle.md)'s, and this task implements it. The
header must not keep advertising "four parity bugs in twenty minutes" as
something the current script can do.

Two things S1a.10.3 leaves for this task to decide, both of them the same
decision wearing different hats:

- the corpus's **`generated`** group is empty-but-kept precisely because this
  is open. If the rewritten fuzzer still writes a throwaway manifest and files
  its cases under that group, the group stays; if it does not, the group goes
  and `ein_corpus::manifest::GROUPS` loses a name.
- the sweep the fuzzer used to drive is `ein-cli/tests/corpus_cli.rs` now, and
  it takes a manifest. Pointing the fuzzer at it — generate a batch, write a
  manifest, sweep it for panics and non-terminations — is the cheapest version
  of properties 1 and 4 in the ledger's L1 list, and it needs no new
  machinery.

### Task T1a.10.4.3 — The benches

`bench_baseline.py` is the Python half of `cargo bench`; with no Python half
it collapses into `cargo bench`. `e2e_baseline.py` keeps its shape — it times
*processes*, and one implementation is still worth timing across builds and
across allocator arms — minus `--impl ein.py` and the PyPy discovery.
`count_work.py` had the same two-column job for the work counters; the ein.rs
half is `ein_core::counters` and survives.

### Task T1a.10.4.4 — The incidental ones

Mechanical: swap the invocation for `ein.rs/target/release/ein` (or an
`$EIN_BIN` defaulting to it) and drop the `PYTHONPATH` dance. `find_dead_defs.py`
and `relation_algebra_examples.py` are Python *tools reading Python source* —
they die with their subject unless someone wants the Rust analogue, and
nothing asks for one.

### Task T1a.10.4.5 — `stdlib_manifest.py`

`MANIFEST.sha256` exists because `ein.py/src/ein/stdlib/` is a build-time copy
that must not drift ([design/11](../design/11_shared_assets.md)). With no
Python package there is **one** stdlib and nothing to drift against, so the
manifest's purpose narrows to "the copy embedded in the binary matches the
checkout". Keep it for that, or retire it and say what replaced the check.

## Notes

- Resist a `--impl` flag that only takes one value. A script that still has
  the shape of a comparison invites someone to look for the other operand.

---

## What shipped — 2026-08-21

Six commits. **`utils/` went from 28 scripts to 17**, and every one of the 17
drives `ein.rs`.

| task | commit | |
|---|---|---|
| T1a.10.4.1 | `6019d21` | `py_oracle.py`, `ir_oracle.py`, `mutant_ein.py` |
| T1a.10.4.3 | `4971055` | `bench_baseline.py`, `count_work.py`; `e2e_baseline.py` single-engine |
| T1a.10.4.4 | `8ef4f65` | six more deletions; `zebra2_trace.sh`, `feature_matrix.py`, `render_examples.sh` |
| T1a.10.4.5 | `25d0f38` | `stdlib_manifest.py` narrowed |
| T1a.10.4.2 | `ec51d42` | the fuzzer, and three findings |
| — | this one | `CLAUDE.md`, the ledger, the record |

### The eleven that went, and why each

| script | class | what happened |
|---|---|---|
| `py_oracle.py` | comparison | CPython's `repr`/`format`/`sorted` behind JSON-Lines. Banked at S1a.10.2 as `ein-core/tests/golden/{repr_values,repr_escapes,float_format}.txt` |
| `ir_oracle.py` | comparison | ein.py's frontend behind the same. Banked as `corpus_shapes.md5` |
| `mutant_ein.py` | comparison | the D3 cut's control. Banked at S1a.10.3 as `ein-infer/tests/event_cut_control.rs` |
| `bench_baseline.py` | comparison | the Python half of `cargo bench`; with one half it *is* `cargo bench` |
| `count_work.py` | comparison | the Python half of the work counters; `ein_core::counters` survives |
| `profile_solve.py` | ein.py measurement | successor `profile_ein_rs.py`, which buckets by the same subsystems on purpose |
| `measure_match_skips.py` | ein.py measurement | sized a semi-naive-matching win in ein.py; ein.rs solved that differently |
| `measure_redundant_firings.py` | ein.py measurement | sized B2.v; its own header already said ein.rs took the win at S1a.6.9, and `fork_split.py` is the ein.rs-side instrument |
| `symmetric_bench.py` | ein.py measurement | successor `feature_matrix.py`'s `no-symmetric-mirror` cell |
| `find_dead_defs.py` | Python source tool | dies with its subject |
| `relation_algebra_examples.py` | Python source tool | ditto |

Nothing on that list was deleted with a claim still riding on it: every row is
either banked by a checked-in test, superseded by a named instrument, or a
tool for a language the repo no longer contains.

### The three that were not mechanical

**`e2e_baseline.py` loses `--impl` rather than gaining a default.** With one
value, a selector "invites someone to look for the other operand" (the stage's
own note). What it keeps is `--bin`, and that is not a consolation: comparing
two *builds* of one engine — an allocator arm, a feature, a `--profile` — is
what every P1a.6 stage actually did with it, and it is the only reason a
process timing still beats `cargo bench`. `$EIN_BIN` moves the default; the
row key is `binary`.

**`feature_matrix.py` loses its cross-check, and the control gets more
important.** `--check` compared verdict, `k`, exhaustion, twelve counters and
goal bindings cell by cell, and it existed because a timing comparison between
engines that explored different numbers of commitments would be meaningless.
There is one engine. What is left to falsify a cell is (a) the counters, read
across *commits* rather than across engines, and (b) the `control` row — a
byte-identical copy of the baseline, measured last, which is the only thing
that prices the method now that no second column can disagree with it.
`config_ok` became the **exit code**: a cell whose generated `(config …)` did
not take is measuring the baseline twice and would otherwise read as an inert
lever.

**`render_examples.sh` is a reduction, and the stage's plan was wrong about
it.** T1a.10.4.4 called it mechanical. Half of it called the *Python API*
directly — `ein.ir.to_dot` and `KnowledgeBase.to_dot` — because `ein ir dot`
and `ein kb dot` were removed from the CLI in P1.11 and never came back; that
is why it needed `PYTHONPATH` as well as an engine. Both renderers are ported
and alive (`ein_render::{ir_dot,kb_dot}`, seventeen views, swept by
`dot_wellformed.rs`), and **nothing outside a test can ask for one**. So the
script renders what the CLI renders — rules, constraints, lattice — and says
in its header what left and what putting it back would take, which is a
decision about the shipping surface (`ein render ir|kb`) rather than a
`utils/` clean-up. Measured after the change: 84 files, 248 rule DOTs, 84
constraints, 79 lattices, 5 lattice skips on the timeout, no warnings.

It also exposed a latent bug: under `nullglob` a glob that matches nothing
expands to *nothing*, and `ls` with no arguments lists the working directory.
A fixture with no rule forms counted **17 "DOT files" that were the repo
root**. `find`, not `ls`.

### T1a.10.4.2 — the fuzzer, and what it found

Five properties, each named in the finding it produces:

| | property | instrument |
|---|---|---|
| 1 | `no-crash` — exit ∈ {0,1,2}, no panic, no signal | the script, per run |
| 2 | `diagnosed` — a refusal says why on stderr | ditto |
| 3 | `terminates` — every run finishes inside `--timeout` | the timeout **is** the instrument; `-2`, the code `corpus_cli.rs` uses for the same thing |
| 4 | `deterministic` — the same argv twice, the same bytes | ditto, with durations masked and **nothing else** masked |
| 5 | `id-order` — the same answer under a permuted interner | `ein-render`'s `id_order_invariance`, via `EIN_ID_FILES` |

**The seam is the interesting part.** Property 5 is
[the ledger's L1 item 3](oracle_ledger.md#6-accepted-loss) — "§5's instrument,
applied to generated input rather than to the corpus" — and §5's instrument is
a `cargo test`. So `id_order_invariance.rs` grew `EIN_ID_FILES=<dir>`: sweep
that directory instead of the corpus, with the two *corpus-shaped* assertions
(the 1 500-pair floor, "some rendering moved") skipped and **named as skipped
in the summary line**. The invariance claim itself is not skipped, which is
the whole point. Shelling out to `cargo test` from Python is deliberate: a
second copy of the sweep in the fuzzer would be a second opinion about what an
observable is, which is precisely the mistake the differential version avoided
by refusing to diff anything itself.

Four things the ledger's four-item list needed, and they are corrected there:
property 2 (`dump → parse → dump`) stays with the frontend fuzzer, which has
its own generator and the dumper this one cannot reach; property 4 (`--jobs`)
has no flag to be invariant under yet; and `diagnosed` and `deterministic` are
additions the list did not have.

**Every instrument was checked against a deliberate break** before being
trusted, which is S1a.10.3's standard and worth keeping: `$EIN_BIN` pointed at
a stub that exits 1 silently (`diagnosed`, 2/2), one that prints its pid
(`deterministic`, 2/2), one that panics (`no-crash`, 1/1 after dedup), and
`--timeout 0.001` (`terminates`, 2/2). `id-order`'s controls are its own.

**Three findings, in the first twenty minutes, all in
[`corpus/fuzz_findings/`](../../../corpus/fuzz_findings/README.md):**

| finding | property | what |
|---|---|---|
| `hrule-reads-not.ein` | `no-crash` | an `(hrule …)` whose `:match` reads the `not` relation trips `debug_assert!` in `Hrules::candidates`. Legal ein-lang; release answers, debug aborts |
| `unsat-core-id-space.ein` | `id-order` | which facts the **unsat core** names, and how many, moves under a permuted id space — six of 45 ops, including `trace[answer]`'s "unsat core: 1 facts" vs "2 facts" |
| `d3-goal-row-order.ein` | `id-order` | on file since 2026-08-20 as a cross-engine diff. Re-derived from a different seed to **the identical seven forms**, then failed under a permuted id space |

The third is the one worth reading twice. It was filed as a
[D3](../divergences.md) consequence — ein.py showed `?x = B`, ein.rs showed
`?x = A` — and the attribution was too narrow rather than wrong. The goal is
satisfied twice in one model, the table prints `rows[0]` of an *unsorted*
match, and D3 changes the order facts entered the KB; so D3 does reach it. But
**the row also moves with D3 held fixed** — one engine, one build,
`fork-delta` off in both runs, only the id space permuted. D3 perturbs the
row; it is not why the row is perturbable, and the difference matters because
after this phase nothing can re-run the D3 reading and `EIN_ID_FILES` can be
re-run forever. `d3-unsat-core.ein` was put through the same check and *is*
D3 (`EIN_FORK_DELTA=0` reproduces the six facts; the id-space sweep is green
on it), so the pair is one of each and both notes now say which.

**None of the three is fixed here.** Each is a semantics decision — what
should an hrule reading `not` do; should the engine report every minimal
unsat core, or a canonical one; should the solve table print every goal row —
and a `utils/` stage is not where those get taken. They are fixtures with
notes, and each becomes a `regression` corpus entry in the commit that settles
it. **This is the first work P1a.10 has produced that is not clean-up**, and
it is a small piece of evidence about [L1](oracle_ledger.md#6-accepted-loss):
the loss is real, and what replaced it is not nothing.

Findings dedup on the panic's site for `no-crash` and on the minimised program
otherwise, seeded from the notes already on disk, so a recorded cause is not
re-filed tomorrow. The session that found the three re-runs at **720 cases,
3 600 runs, 0 findings, 67 duplicates suppressed** — which is what "empty is
the steady state" has to look like from the inside.

### The `generated` group, decided

**Gone.** `GROUPS` is six names. It existed for the throwaway manifest the
fuzzer wrote to hand a batch to `ein-conformance`; the rewritten fuzzer runs
the binary directly and writes no manifest. The deeper reason is that it could
never have held anything: **a corpus entry is a file the engine is
*permanently* exercised over**, and a generated case lives for milliseconds. A
find worth keeping goes to `fuzz_findings/` and, once its expectation settles,
becomes a `regression` entry with a name. `corpus/` now has no empty group —
the other one, `golden`, went at S1a.10.3 for the same reason: an empty group
is a question with a home, and both questions are answered.

The D2 exclusions went with it. The generator avoided negative `:assert` heads
with int arguments, and `seed_corpus` skipped the two D2 fixtures, so that the
fuzzer would stop re-finding the ledger's own accepted divergence. There is
nothing to diverge from.

### T1a.10.4.5 — the manifest, and a claim that had to be measured

The task offered "keep it for that, or retire it and say what replaced the
check". The narrowed purpose — *the copy embedded in the binary matches the
checkout* — is already owned by `ein-ir`'s
`the_embedded_copy_matches_the_manifest`, and **it is not stale-able**, which
the task could not assume: `include_dir!` registers each module as a build
dependency, so appending a comment to `stdlib/algebra.ein` turns
`cargo test -p ein-ir --lib stdlib` red with no other change. Measured, not
argued.

So the script keeps the half `cargo test` structurally cannot do — **writing**
the manifest, since a test that rewrote the file it checks would check nothing
— and keeps verify for the two things it does better: it names the drift per
module where the assertion names the first one it reaches, and it answers in
milliseconds with no toolchain, which is why it is the per-commit tier's first
step. `--against DIR` and the `ein.py/src/ein/stdlib` fallback in
`stdlib_dir()` are gone; the fallback was the corpus completeness check's
defect in miniature, turning "the stdlib is not where it should be" into
"checked a different directory, all fine".

## Acceptance, checked

| criterion | |
|---|---|
| every script runs against ein.rs or is gone; none imports a module that no longer exists | 17 scripts, 11 deleted. `git grep -l 'from ein\|import ein\b' utils/` is empty |
| the M1a measurement set still works | `bench_env.sh`, `e2e_baseline.py`, `profile_ein_rs.py`, `criterion_table.py` all re-run; `e2e_baseline.py` smoke-tested on both its default and `--bin` paths |
| the Python columns are historical constants, and the documents say which numbers are frozen | a **live / frozen / gone** table at the top of `baseline.md`, whose "Reproducing all of it" appendix is rewritten to be runnable and says it is the one that is maintained; `design/README.md § Measured`, `design/12 §4` and `features.md § Refresh` each say the same thing about their own column |
| `CLAUDE.md`'s `utils/` description matches what is there | rewritten: the three groups, the `$EIN_BIN` convention, the three exceptions to it, and "none takes an `--impl`" |
| a deleted script's result survives where the result was the point | **the acceptance's example rests on a false premise**, and it is worth saying so: `utils/feature_matrix_results.json` is *git-ignored* — it could not outlive anything. The lever matrix's committed record is `docs/kernel/inference/features.md`, which is where the ein.py column is now frozen, and `feature_matrix.py` survives anyway |

**Not done here, and named:** the three fuzz findings are questions, not fixes
(above). `run_tests.sh`, `.github/workflows/nightly.yml`'s `full-suite` and
`packaging` jobs, and every `ein.py/` link in `README.md`, `docs/api/` and
`docs/kernel/` are [S1a.10.5](s1a.10.5_removal.md)'s and
[S1a.10.6](s1a.10.6_docs.md)'s; this stage touched them only where its own
deletions made them dangle.

**The gate:** `cargo test --workspace` — **542 passed**, 0 failed.
`cargo fmt --check` and `cargo clippy --workspace --all-targets -- -D warnings`
clean. `python3 utils/stdlib_manifest.py` — 7 modules match.
