# S1c.1.1 — What the stdlib promises, and what is exercised

**Phase:** P1c.1 (stdlib conformance)
**Estimate:** 3 days

**Status: shipped 2026-08-23**, the first stage of
[M1c](../README.md). The record is [`stdlib_census.md`](stdlib_census.md); the
instrument that produced it is
[`utils/stdlib_census.py`](../../../utils/stdlib_census.py), the nineteenth
script in `utils/` and the first check that is about the *standard library*
rather than the engine.

| finding | number |
|---|---|
| stdlib rules declared | **73**, over six modules |
| rules **no corpus run activates** | **38** — 52 %, of which **33** are never even loaded |
| rules activated by **exactly one entry** | **23**, and `examples/zebra.ein` is that entry for **20** |
| untested **or** held up by one file | **61 of 73 — 84 %** |
| coverage if `examples/zebra.ein` were dropped | **35 rules → 15** |
| modules at **zero** coverage | **two** — `std.typing`, `std.closure` |
| modules at **full** coverage | **one** — `std.slots`, all eighteen by one file |
| rules that fire and derive **nothing** | **3** — `functional`, `injective`, `slot-prune-bwd` |
| rules that read zero only because of the `normal` elision | **3** — the S1a.7.0 trap, measured |
| example files declaring their **own** copy of a stdlib rule name | **25** — unfiltered, `symmetric` reads 112 271 productive firings over 22 entries against the true **1 084 over 7** |
| declarations that are another declaration renamed | **4 pairs**, one of which differs only in **priority** (220 vs 110) |
| rules in the zero set that are *unreachable* | **0** — the outcome this stage's T1c.1.1.3 went looking for and did not find |
| handed to S1c.1.4 | the estimate: **4 days → 6**, and one task that was not in the plan — a second `std.slots` program |

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

### Task T1c.1.1.1 — The promise inventory

By hand, per module, one sentence per rule, checked against the rule body
rather than against its `:why`. Seven modules; `std.algebra` and
`std.bijection` carry most of it.

### Task T1c.1.1.2 — The firing census

A sweep over the corpus with `--events`, counting `fire` by `rule`. Cheap:
ein.rs runs the whole corpus in seconds. Report per rule and per module.

Watch for the trap the [S1a.7.0](../../../docs/history/m1a_rust/README.md#s1a70--the-speculation-audit)
audit hit: **`normal` event level elides redundant firings**, so a rule that
only ever re-derives an existing fact reads as zero. Run at `verbose` and
report both counts — the difference is itself interesting, since a rule whose
firings are *all* redundant is doing no work in this corpus.

### Task T1c.1.1.3 — What activates a rule that nothing activates

For each zero-firing rule, the smallest program that would. Some will be
"nothing in the corpus declares `(surjective R)` on a relation where it can
fire"; some will be "this rule is unreachable given the others' priorities",
which is a finding.

### Task T1c.1.1.4 — The negative shapes

Per rule, what *should not* fire. Derived from the guards: every `neq`, every
`absent`, every `forall` in a premise is a case where firing is wrong, and the
disjunctive-prune bug was precisely a guard that made the rule *not* fire
where it should. Both directions belong in the table.

## Notes

- The output of this stage is what [S1c.1.4](s1c.1.4_stdlib_corpus.md)
  writes tests against, and its size decides that stage's estimate. If the
  zero-firing set is large, say so before committing to four days.
  **It is large — 38 of 73 — and the answer is six days, not four**, for a
  reason the stage did not anticipate: the zero set is *cheap* (small generic
  rules that three facts activate) and the expensive item is not in it at all.
  `std.slots` is the module at 100 % rule coverage, and every one of its
  eighteen rules is activated by `examples/zebra.ein` and by nothing else.
  [`stdlib_census.md` §10](stdlib_census.md#10-what-this-does-to-s1c14).
