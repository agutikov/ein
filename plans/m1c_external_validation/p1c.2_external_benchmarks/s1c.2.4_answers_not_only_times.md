# S1c.2.4 — Answers, not only times

**Phase:** P1c.2 (External benchmarks)
**Estimate:** 2 days
**Depends on:** [S1c.2.3](s1c.2.3_the_runner.md)

## Context

The validation half of the phase, and the reason it sits in
[M1c](../README.md) rather than in a performance milestone. Six systems that
share no code with Ein and no code with each other, answering the same
question, is the strongest check the project can buy — stronger than the
two-engine parity it replaces, because two implementations of the *same
algorithm* fail the same way and CDCL, CLP(FD), bottom-up Datalog and a
proof assistant do not.

The bug that motivates it is on the record: `disjunctive-prune`'s guard was
wrong for a year, byte parity signed off on it five phases running, and an
independent enumeration found it in an afternoon. That enumeration was a
throwaway script. This stage makes it a permanent, re-runnable cell in a
table.

## Acceptance

- **A canonical answer form per problem**, system-independent: for the zebra
  family a sorted assignment of (attribute, house) pairs; for n-queens the
  column index per row. Systems print wildly different things and the
  comparison happens after normalisation, never before.
- **An adapter per (problem, system)** that maps output into that form, and
  each adapter is tested against a hand-checked answer — including a
  deliberately *wrong* one, so an adapter that always reports agreement fails.
  [S1a.6.6](../../m1a_rust/p1a.6_performance/s1a.6.6_differential_fuzzer.md)'s
  lesson, "the fuzzer's own three controls each failed once first", applies
  directly.
- **Model sets are compared as sets**, and each system's *enumeration mode* is
  recorded with its exhaustiveness claim, because they differ in kind: Clingo
  `--models 0` enumerates stable models; Z3 needs a blocking-clause loop and
  stops when the formula turns unsat; SWI-Prolog's `findall` backtracks over
  the whole search space; `ein solve -e` exhausts the commitment lattice. Four
  different arguments for "that is all of them", and the report says which one
  each column carries.
- **`zebra2-minus-15`'s 32 models are confirmed by at least two non-Ein
  systems** — or the disagreement is the phase's headline finding.
- **Confirmed answers are written back into the `.ein` file as `:expect`**
  ([P1c.1](../p1c.1_stdlib_conformance/README.md)'s form), with a comment
  naming the systems that confirmed it and the date. From then on `ein test`
  re-checks the answer on a machine with no solver installed at all — the
  external systems establish the answer, they do not keep it.
- **A disagreement is a first-class outcome with a written protocol**: check
  the encodings first (rule 2 of [S1c.2.1](s1c.2.1_problem_corpus.md) exists
  because a bad encoding is the likelier cause), then the adapters, then the
  engines. A confirmed disagreement involving Ein becomes a fixture under
  `examples/ein-bugs/` and a bug report, not a footnote in a table.

## Tasks

### Task T1c.2.4.1 — Canonical forms
### Task T1c.2.4.2 — The adapters, and the adapters' own tests
### Task T1c.2.4.3 — Enumeration modes, per system

The part where the systems are least comparable and most interesting. Write
down, per system, what its "all models" means and what it costs to ask —
including the ones where asking is awkward (an SMT blocking-clause loop is
*the benchmark author's* algorithm, not the solver's, and that has to be said
out loud).

### Task T1c.2.4.4 — The writeback into `:expect`
### Task T1c.2.4.5 — The disagreement protocol

## Notes

- Agreement across systems is evidence, not proof: all six could share a
  misreading of the puzzle's English. The zebra family's answer is famous and
  independently published, which is why it is the corpus's anchor — and it is
  why `zebra2-minus-15`, whose 32 models are *not* published anywhere, is the
  entry where cross-checking does actual work.
- This is also the only stage in the milestone that can find a bug in Ein
  today. Budget for that: if it finds one, the fix is not in this phase's
  estimate.
