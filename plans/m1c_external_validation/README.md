# M1c — External validation

**Estimate:** ~3 weeks — 1 phase, 5 stages, 15 days of stage estimates
(13 until [S1c.1.1](p1c.1_stdlib_conformance/s1c.1.1_what_the_stdlib_promises.md)
measured what S1c.1.4 is actually up against).
**Status:** **started 2026-08-23** — S1c.1.1 and S1c.1.2 shipped, three stages to go.
**Created 2026-08-21** at the user's direction — one evening after
the same batch put P1a.10–12 into M1a — out of one phase that was never the
Rust port and one that had nowhere to live.
[P1c.1](p1c.1_stdlib_conformance/README.md) is M1a's ex-P1a.11 — stages and
dependencies unchanged, with only the two paragraphs that named the wrong
milestone rewritten; the estimates were unchanged too, until S1c.1.1 measured
S1c.1.4's work list and moved it from 4 days to 6. The second phase, P1c.2, was **promoted to
[M10](../m10_external_benchmarks/README.md) on 2026-08-23** and took its five
stages and its three method questions with it; what stays here is the half
that runs with no external tool installed.
**Depends on:** [M1a](../../docs/history/m1a_rust/README.md) — [P1a.10](../../docs/history/m1a_rust/README.md#p1a10--one-implementation)
for P1c.1: a new surface form is written once when there is one
implementation and twice when there are two.
**Blocks:** nothing on the critical path. [M5](../m5_presentation/README.md)
Track A is the consumer: its "head-to-head numbers where applicable" are
this milestone's output, and the paper is not where a benchmark should be run
for the first time.

---

## The thesis

**Every check this repo has is relative.** The conformance tiers compared two
engines; the goldens compare ein.rs to its own past; after
[P1a.10](../../docs/history/m1a_rust/README.md#p1a10--one-implementation) the second engine
is gone, so what is left compares ein.rs to yesterday's ein.rs. All of it
answers *did this change?* None of it answers *is this right?*

There are exactly two ways to answer the second question, and this milestone
owns the first of them:

1. **What the rules say they do.** An expectation written next to a rule, run
   by the engine — [P1c.1](p1c.1_stdlib_conformance/README.md). *This
   milestone.*
2. **What other systems answer.** The same problem stated for Z3, CVC5,
   SWI-Prolog, Soufflé, Clingo and Lean, run by one harness, compared on the
   *answer* first and the clock second —
   [M10](../m10_external_benchmarks/README.md), a phase here until
   2026-08-23.

The precedent is specific, recent and expensive. `disjunctive-prune`'s
`(neq ?h_other ?h1)` guard was wrong for a year — through five phases of
byte-exact parity — and what found it was an **independent enumeration** of a
puzzle's models, written outside the engine on the day. Both engines agreed
with each other the whole time, and agreement was all anything checked.
P1c.1 makes that kind of check cheap for a *rule*; M10 makes it permanent
for a *puzzle*.

### Splitting them did not split the pipeline

M10's ground truth lands in P1c.1's form. When Clingo enumerates 32 models
of `zebra2-minus-15` and Z3's blocking-clause loop agrees, that answer is
written into the `.ein` file as an `:expect`, and from then on `ein test`
re-checks it on a machine with no external solver installed at all. **The
external tools are needed to establish the answer, not to keep it.** That is
why the form is written here and the campaign that fills it is a milestone of
its own: P1c.1 still goes first, and it goes first for a reason that does not
depend on the two sharing a directory.

---

## Phases

| phase | title | stages | est. | gate |
|---|---|---|---|---|
| [P1c.1](p1c.1_stdlib_conformance/README.md) | stdlib conformance — `:expect` on `query`, `ein test`, a corpus per rule | 5 (2 shipped) | 3 w | every stdlib rule has a program that activates it and states what it derives |

5 stages, 15 days of stage estimates ≈ 3 weeks. The other five went to
[M10](../m10_external_benchmarks/README.md) on 2026-08-23.

### The gap, measured — S1c.1.1, 2026-08-23

The thesis above was an argument. The first stage turned it into a number, and
the number is worse than the argument assumed:
[**38 of the stdlib's 73 rules never fire**](p1c.1_stdlib_conformance/stdlib_census.md)
in any of 128 corpus entries × 400 inference runs — 33 of them never even
loaded — and 23 more are activated by exactly one entry, `examples/zebra.ein`
being that entry for 20. Two modules, `std.typing` and `std.closure`, have
never executed a rule at all. **84 % of the standard library is either untested
or resting on a single file**, and the whole relative and Boolean layer of the
relation algebra — composition, meet, join, complement, top, identity,
difference, and every equational lemma — has never been run.

That is what "not contradicted" means in practice, and it is why the rest of
this milestone is worth its three weeks.

### The form, built — S1c.1.2, 2026-08-23

`:expect` on `query`
([Q-M1c.1](open_questions.md#q-m1c1--how-does-a-program-state-what-it-expects)
closed to option (c)), with several `(query …)` blocks per file and the
last-one-silently-wins discard gone:

```lisp
(query :goal   (pet-loc Zebra ?h)
       :expect (model (pet-loc Zebra House-5) (pet-loc Fox House-1)
                      (not (pet-loc Zebra House-1))))
```

**Naming a relation closes it** — the listed `pet-loc` facts are that
relation's complete extent, so a *surplus* fact fails, which is the case a
per-fact assertion cannot catch and the shape of the bug this milestone is
written around. `(or (model …) …)` compares model **sets** with `k` implied by
the count; `none` is `Contradiction`. `ein solve` exits 1 when the claim is
false, so a file carrying one is a test with no harness around it —
[`examples/features/10_expect.ein`](../../examples/features/10_expect.ein) is
the worked fixture. What it cost: **0** new grammar productions, **0** goldens
moved, 14 call sites, and five new load-time refusals which are the repo's
first diagnostics with no Python counterpart.

## Acceptance for the milestone

- **No stdlib rule rests only on self-agreement.** Every one has a program
  that activates it and states what it derives — and the statement is
  machine-checked, not prose.
- The expectations are **checked in as `:expect`**, so an answer established
  anywhere — by hand, by argument, or by the six systems of
  [M10](../m10_external_benchmarks/README.md) — survives the absence of
  whatever produced it.
- **A missing tool is reported, never skipped past.**
  [S1a.10.1](../../docs/history/m1a_rust/README.md#s1a101--bank-what-only-the-oracle-proves)
  found 42 tests that started a Python process and skipped invisibly when one
  would not start; `ein test` has the same failure mode available to it and
  must not take it.
- The milestone's other half — every benchmark problem's answer confirmed by
  a system that is not Ein, and the report that says where Ein loses — is
  [M10](../m10_external_benchmarks/README.md)'s acceptance now.

## Non-goals

- **A test framework.** `ein test` runs what a program states about itself;
  it does not grow fixtures, mocks or a discovery protocol. The corpus
  already knows how to enumerate files.
- **New reasoning features.** If an expectation cannot be *stated* — because
  ein-lang has no way to say it — that is a finding for
  [M1d](../m1d_satisfiability/README.md) or a followup, not a stage here.
- **Everything the benchmarks are not.** A benchmark suite, a table built to
  be won, and Ein's own performance work were non-goals of this milestone
  while [M10](../m10_external_benchmarks/README.md) was a phase of it; they
  are M10's to disclaim now, and it does.

## Open questions

[`open_questions.md`](open_questions.md) — `Q-M1c.<n>`. Two remain here, both
inherited from M1a with P1c.1 and keeping their text: **Q-M1c.1** (ex
Q-M1a.19) how a program states what it expects, and **Q-M1c.2** (ex Q-M1a.20)
what an expectation may say. The three about *method* — a fair encoding, a
proof assistant in a timing table, where the harness lives — went to
[M10](../m10_external_benchmarks/README.md) on 2026-08-23 as **Q-M10.1–3**,
and the index here keeps their old ids as redirects.

## Cross-links

- [`p1c.1_stdlib_conformance/stdlib_census.md`](p1c.1_stdlib_conformance/stdlib_census.md)
  — the milestone's first output: what 73 rules promise, and what the corpus
  activates. Re-taken by [`utils/stdlib_census.py`](../../utils/stdlib_census.py)
- [`stdlib/`](../../stdlib/) — the seven modules P1c.1 puts under test
- [`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md) — the solver
  catalogue M10's system list is drawn from; [`03`](../../docs/lib/03-theorem-proving-formal-methods.md)
  for Lean
- [`smt/`](../../smt/) — `4-queens.smt` and `einstain-problem.smt`, hand-written
  in 2021 and kept as *encoding examples*: the corpus M10 builds is what
  that scratch directory was gesturing at
- [`utils/`](../../utils/) — `bench_env.sh` and `e2e_baseline.py`, the
  measurement discipline M10 inherits rather than re-invents
- [M5](../m5_presentation/README.md) Track A / Track B — the consumer
- [M1d](../m1d_satisfiability/README.md) — the sibling created the same day;
  `zebra2-minus-15`'s 32 models are M10's cross-check and M1d's subject
