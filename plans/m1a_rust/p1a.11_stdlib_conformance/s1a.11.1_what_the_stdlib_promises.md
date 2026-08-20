# S1a.11.1 — What the stdlib promises, and what is exercised

**Phase:** P1a.11 (stdlib conformance)
**Estimate:** 3 days

## Context

Before writing a test form, find out what needs testing. Two questions, and
only the second needs new code:

1. **What does each rule promise?** Its match, its guard, its assert, its
   priority, and — the part that is nowhere written down — *what it is for*.
   `std.elim`'s `domain-elimination` asserts a positive once every alternative
   is excluded; `no-room-left` asserts `(false)` once all are. Those are two
   different promises about the same premise shape.
2. **Which rules does the corpus actually activate, and how often?** This is
   measurable: `--events` already reports `fire` with the rule name, so a sweep
   over every corpus entry gives a firing count per rule. A rule with a count
   of zero is untested, whatever the suite says.

## Acceptance

- A table: **rule × (module, promise, corpus firings, corpus entries that
  activate it)**. Generated, not hand-written, so it can be re-run.
- The **zero-firing set** named explicitly. That is the phase's first work
  item and probably its most valuable output.
- The **low-firing set** — activated by exactly one entry — named too: those
  are tested by one puzzle's accident, and if that puzzle changes they become
  untested silently.
- Rules whose promise cannot be stated in one sentence are flagged. That is a
  finding about the rule, not about the table.

## Tasks

### Task T1a.11.1.1 — The promise inventory

By hand, per module, one sentence per rule, checked against the rule body
rather than against its `:why`. Seven modules; `std.algebra` and
`std.bijection` carry most of it.

### Task T1a.11.1.2 — The firing census

A sweep over the corpus with `--events`, counting `fire` by `rule`. Cheap:
ein.rs runs the whole corpus in seconds. Report per rule and per module.

Watch for the trap the [S1a.7.0](../p1a.7_parallelism/s1a.7.0_speculation_audit.md)
audit hit: **`normal` event level elides redundant firings**, so a rule that
only ever re-derives an existing fact reads as zero. Run at `verbose` and
report both counts — the difference is itself interesting, since a rule whose
firings are *all* redundant is doing no work in this corpus.

### Task T1a.11.1.3 — What activates a rule that nothing activates

For each zero-firing rule, the smallest program that would. Some will be
"nothing in the corpus declares `(surjective R)` on a relation where it can
fire"; some will be "this rule is unreachable given the others' priorities",
which is a finding.

### Task T1a.11.1.4 — The negative shapes

Per rule, what *should not* fire. Derived from the guards: every `neq`, every
`absent`, every `forall` in a premise is a case where firing is wrong, and the
disjunctive-prune bug was precisely a guard that made the rule *not* fire
where it should. Both directions belong in the table.

## Notes

- The output of this stage is what [S1a.11.4](s1a.11.4_stdlib_corpus.md)
  writes tests against, and its size decides that stage's estimate. If the
  zero-firing set is large, say so before committing to four days.
