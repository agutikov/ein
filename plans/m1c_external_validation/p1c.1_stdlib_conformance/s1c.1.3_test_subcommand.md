# S1c.1.3 — `ein test`

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 2 days
**Depends on:** [S1c.1.2](s1c.1.2_test_form.md)

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
  [Q-M1a.13](../../m1a_rust/open_questions.md#q-m1a13--argparse-surface-parity) settled.

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
[S1a.6.6](../../m1a_rust/p1a.6_performance/s1a.6.6_differential_fuzzer.md) lesson — "the
fuzzer's own three controls each failed once first" — is the precedent.

## Notes

- Resist making `ein test` a general test framework. It evaluates the
  expectations a program carries; it does not have setup, teardown, fixtures
  or parameterisation. If a rule needs those to be tested, the interesting
  finding is about the rule.
