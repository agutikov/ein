# S1c.1.3 — `ein test`

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 2 days
**Depends on:** [S1c.1.2](s1c.1.2_test_form.md)

**Status: shipped 2026-08-24.** The fourth subcommand exists and the corpus
runs it. Two of the five tasks were **already decided** by the stage before
this one, and saying so is most of what a reader of this document needs.

| finding | number |
|---|---|
| the subcommand | `ein test <PATH>…`, **8** options — and no `-n`, no `--exhaustive` |
| the vocabulary below (`:derives`, `:absent`, `:fires`, `:verdict`) | **none of it shipped.** S1c.1.2 chose option (c), so there is **one** expectation kind and T1c.1.3.2's "one pass per expectation kind" is one pass |
| …and T1c.1.3.2's redundant-firing question with it | moot: route is parked, [stdlib_census §8](stdlib_census.md#8-four-declarations-are-two-rules) decided it, and `:fires` never arrived to have a reading |
| what "only the work the expectations need runs" became | **a query with no `:expect` is never solved.** `ein test examples/features/` checks 3 of 12 files and never enters `04_open.ein`'s unbounded search — the entry the corpus marks as "a run nobody can finish is not coverage" |
| exhausting | **the behaviour, not a flag.** An expectation is a claim about the exhausted answer, so a `-n` on this command would be a way to ask for `NOT CHECKED` |
| the exit codes, and where they leave `solve` | 0 / 1 / 2 as the acceptance asks — and **1 is "a claim is false", so a load error takes 2**, which `solve` gives 1. Deliberate: a runner that cannot tell a broken file from a false claim is what T1c.1.3.5 is against |
| a bug in S1c.1.2's checker, found by the exhaustive default | **a `Contradiction` from a truncated search was reported as a hard failure.** `k = 0` from a capped lattice is "no model within the cap"; it is now `NotChecked`, and `--max-set-size` is the only thing left that can truncate a `test` run |
| T1c.1.3.3's "the derivation of the fact that should not have been there" | landed in **`ein-infer::expect`**, not in the CLI — so `ein solve`'s `:expect FAILED` block grew it too. One level of premises: `…(p B A) is derived by symmetric from (p A B)` |
| …and its other half, "prints the k **and the models' query bindings**" | a count mismatch now projects every model through the query's own `:goal`, sorted (§6's row order is not observable): `model 2 of 2: ?slot=S1 ?who=Bob; ?slot=S2 ?who=Ann`. `expect::check` takes `&mut Terms` for it, because projecting a goal interns |
| tests added | **29** — 25 in `ein-cli/tests/test_cli.rs`, 4 in `ein-infer/tests/expect_semantics.rs` |
| corpus cells | **3**, one per verdict fixture; 3 golden lines, all `0` |
| the help surface | 40 options across 8 parsers → **48 across 9** |

**What the stage document below does not know.** It was written on 2026-08-20,
*before* [S1c.1.2](s1c.1.2_test_form.md) settled the form, so it describes
option (b)'s vocabulary — a `(test …)` head with `:derives` / `:absent` /
`:fires` / `:verdict` keys. Option (c) shipped instead: one keyword, one
expectation kind, model-shaped. The acceptance criteria survive that
translation intact and every one of them is met; the *tasks* do not, and T1c.1.3.2
is the one that mostly evaporated. Read the acceptance list as the contract and
the task list as the plan it was written against.

## Context

The fourth subcommand: `ein {render,saturate,solve,test}`. It loads a program,
runs whatever its expectations need — saturation for `:derives` / `:absent`,
the search for `:verdict` — evaluates them, and exits 0 or 1.

The point of the user's framing is that **nothing reads output**. Today,
checking that a rule works means running `solve` and looking, or diffing
against a golden. `ein test` makes the expectation the program's own, and the
result a status code.

## Acceptance

- `ein test <file>` — exit 0 if every expectation holds, 1 if any fails,
  2 for a load/usage error (matching the other subcommands' convention).
- Failure output names **which** expectation failed and what was found
  instead. A fact that should be derived and is not prints the fact; a
  `:verdict` that came out `Ambiguity` prints the k and the models' query
  bindings.
- `ein test <dir>` (or a glob) runs a corpus and reports a summary line. This
  is what the gate calls.
- **Only the work the expectations need runs.** A file with only `:derives`
  never enters the search — otherwise a stdlib test on a program with an open
  hypothesis space costs what
  [`features/04_open.ein`](../../../examples/features/04_open.ein) costs, which
  the corpus already marks as "a run nobody can finish is not coverage".
- `--events` / `--json-summary` still work under `test`, because a failing
  expectation is exactly when someone wants the trace.
- The help surface grows one subcommand and stays in the shape
  [Q-M1a.13](../../../docs/history/m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity) settled.

## Tasks

### Task T1c.1.3.1 — The subcommand
### Task T1c.1.3.2 — The evaluator

One pass per expectation kind. `:derives` / `:absent` are a fact-store probe
against the saturated root. `:fires` / `:does-not-fire` read the firing list —
**and must decide about redundant firings**: a rule that re-derives an
existing fact has fired, but at `normal` event level it is invisible.
`:fires` should mean "this rule produced this state", which is the verbose
sense. Say which, in the docs, because the two readings disagree.

### Task T1c.1.3.3 — Failure reporting

The output is read by a person debugging a rule, so it shows the expectation,
the actual, and enough context to act — for `:absent`, the derivation of the
fact that should not have been there. `explain` already computes that.

### Task T1c.1.3.4 — Directory mode and the summary
### Task T1c.1.3.5 — Tests for the tester

A test runner that reports success on a broken expectation is the worst
possible outcome here, so: fixtures that must fail, checked for *failing*,
with the right exit code and the right message. The
[S1a.6.6](../../../docs/history/m1a_rust/README.md#s1a66--the-differential-fuzzer) lesson — "the
fuzzer's own three controls each failed once first" — is the precedent.

## Notes

- Resist making `ein test` a general test framework. It evaluates the
  expectations a program carries; it does not have setup, teardown, fixtures
  or parameterisation. If a rule needs those to be tested, the interesting
  finding is about the rule.

## What shipped, in more detail

### The surface, and the two options that are absent

Eight options: `-m/--max-set-size`, `-T/--max-time`, `-E/--max-enterings`,
`-v/--verbose`, `-q/--quiet`, `--events`, `--events-level`, `--json-summary`.
The three engine knobs are the ones that decide whether a run *can finish*;
everything else `solve` offers is a way to ask a different question, and this
command asks the one the file already asked.

**No `-n` and no `--exhaustive`** is the load-bearing absence. `solve` defaults
to `-n 1` and is right to — it is asked for an answer. `test` is asked whether a
claim holds, and a stopped search establishes a lower bound on `k`, which
confirms no verdict at all. `examples/features/11_expect_ambiguity.ein` is the
demonstration: under `solve` it needs `-e` and the corpus entry says so; under
`test` it needs nothing.

### "Only the work the expectations need runs"

One load per file finds out which of its queries carry an `:expect`; the ones
that do not are never solved, and a file with none costs exactly that load.
`ein test examples/features/` is the case worth stating — 12 files, 3
expectations, and `04_open.ein` untouched. `a_query_with_no_expectation_is_never_solved`
holds it, with a wall-clock assertion two orders of magnitude clear of what
that entry costs the sweep.

### The three ways a run produces no verdict, all of them exit 2

A load error, a budget abort, and a selection that checked nothing — the last
being the shape M1c's acceptance names ("a missing tool is reported, never
skipped past") as it appears here: a corpus with no tests in it must not report
green. `ein test` on a file whose queries carry no `:expect` prints
`nothing to check` on stderr and exits 2; so does a directory with no `.ein`
in it.

### Failure reporting: two lines a reader used to have to go and get

T1c.1.3.3 asked for "the expectation, the actual, and enough context to act",
and named two cases. Both landed in **`ein-infer::expect`** rather than in the
CLI, so `ein solve`'s `:expect FAILED` block grew them too:

- **A surplus fact says which rule put it there.** `(p B A)` in a closed
  relation is a failure; `…(p B A) is derived by symmetric from (p A B)` is the
  step at which `disjunctive-prune`'s guard bug was actually found. One level
  of premises, not a walk — the rest of the chain is `--trace`'s.
- **A count mismatch says what the models were.** "expected Solution with
  k = 1, got Ambiguity with k = 2" used to end there, sending the reader back
  to `solve -e` to find out what the second model was. Each model is now
  projected through the query's own `:goal` and sorted, because
  [`defined_behaviour.md` §6](../../../docs/kernel/defined_behaviour.md) files
  the row order a goal projection produces as under-determined and a
  diagnostic that inherited it could not be diffed.

The second cost an API change: `expect::check` takes `&mut Terms`, because
compiling a goal pattern interns. Three call sites.

### The bug the exhaustive default found

S1c.1.2's checker short-circuited on `Verdict::Contradiction` before reaching
the "too few models, and the search was not exhausted" arm, so a claim of two
models against a depth-capped search came back **FAILED** — a refutation on the
strength of a search that stopped. `MonotonicStats::exhausted`'s own
documentation says why that is wrong: "a `k = 0` from a truncated one is 'no
model within the cap', not proven unsat". It is `NotChecked` now, and it names
the cap to raise.

It was unreachable in practice until this stage, because `solve`'s `-n 1` never
produces a truncated `k = 0` — only `--max-set-size` does, and no corpus entry
passed one. `a_contradiction_from_a_truncated_search_is_not_checked` and
`an_exhausted_contradiction_still_refutes_a_model_claim` are the pair: the
guard must not have turned a refutation into a shrug.

### What it does not do, and where each lives

- **No coverage gate.** "Adding a rule to the stdlib without a test fails the
  gate" is [S1c.1.5](s1c.1.5_gate.md)'s, and `utils/stdlib_census.py` is the
  instrument. `ein test` reports what it checked; it does not know what it
  should have.
- **No stdlib corpus.** Three fixtures exist, one per verdict, and they are
  S1c.1.2's. The programs that activate 73 rules are
  [S1c.1.4](s1c.1.4_stdlib_corpus.md)'s, and the census's §6 work list is what
  they are written from.
- **No `--jobs`.** A file at a time, one thread. Parallelising a test run is a
  thing to do when a test run is slow, and the whole `features/` directory is
  0.00 s.

### Two things about the walk, neither of them interesting until they bite

A directory walk **does not follow a symlinked directory** — `walkdir`'s
default, and for its reason: a link back up the tree is an infinite walk, and a
gate command that hangs is a worse failure than one that misses a file nobody
asked it for. A path named explicitly is still followed, because naming it is
asking for it. And a program named twice — `ein test dir dir/x.ein` — is one
program, so the summary line counts the corpus rather than the argv.
