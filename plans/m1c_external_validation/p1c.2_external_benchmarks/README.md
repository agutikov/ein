# P1c.2 — External benchmarks

**Milestone:** [M1c — External validation](../README.md)
**Estimate:** 2.5 weeks (12 days of stages)
**Depends on:** [P1c.1](../p1c.1_stdlib_conformance/README.md) — not to run,
but to *keep*: the answers this phase establishes are written back into the
`.ein` files as `:expect`, and that form is P1c.1's.
Also on [M1a](../../m1a_rust/README.md) having shipped its
[performance phase](../../m1a_rust/p1a.6_performance/README.md): a
comparison run against the pre-S1a.6 engine measures a version nobody ships.

## Goal

**The same problem, stated once for each system, run by one harness, reported
in one table — answers first, times second.** Z3, CVC5, SWI-Prolog + CLP(FD),
Soufflé, Clingo and Lean 4 against `ein solve`, on a small corpus that every
one of them can express.

## Why a comparison, when M3 was dropped

M3 (SMT integration) was dropped 2026-08-18 and stays dropped: Ein has no
solver back-end, hands no `(hard-slice …)` to anybody, and this phase does
not reopen that. The drop note already made the distinction — what survived
the milestone was [`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md)
"as external-tech catalogue and M5 Track A's *comparison* axis" — and the
sharpening it added is the reason to run the benchmarks at all:

> Ein never hands a slice to a solver … which sharpens the question rather
> than removing it: **the graph engine has to stand on its own numbers.**

There are no such numbers today. The engine's entire performance record
([baseline.md](../../m1a_rust/p1a.6_performance/baseline.md)) is Ein against
Ein: ein.rs against ein.py, against PyPy, against its own previous commit.
165× a Python implementation of the same algorithm says nothing about how the
algorithm compares to a CDCL solver on the same puzzle.

## Two products, and the first one is not the clock

1. **Answers.** Every system's answer for every problem, compared. For the
   under-determined entry that means the model *set*: Clingo's `--models 0`,
   a Z3 blocking-clause loop and a Prolog `findall` each enumerate
   independently, and if they agree, the count is established by three
   systems that share no code with Ein and no code with each other. This is
   the [`disjunctive-prune`](../p1c.1_stdlib_conformance/README.md) check
   made permanent: that bug survived a year of two engines agreeing and died
   to one independent enumeration.
2. **Times, and what had to be said to get them.** Wall clock as a process,
   peak RSS, and the encoding column — how many lines, and *what the language
   made the author state*. On n-queens that column is the interesting one:
   the kernel has no arithmetic ([Q17](../../open_questions.md#q17--spatial-relation-formalisation)
   — "no integer-arithmetic position lattice"), so diagonals arrive as an
   extensional relation the way `right-of` does, while Z3 writes
   `(!= q1 (+ q2 1))` and is done. That is a real difference between the
   systems and the table should show it rather than hide it in a ratio.

## Stages

| stage | title | est. |
|---|---|---|
| [S1c.2.1](s1c.2.1_problem_corpus.md) | The problem corpus, and what a fair encoding is | 3 d |
| [S1c.2.2](s1c.2.2_systems_and_install.md) | The systems: versions, and how they install | 2 d |
| [S1c.2.3](s1c.2.3_the_runner.md) | The runner | 3 d |
| [S1c.2.4](s1c.2.4_answers_not_only_times.md) | Answers, not only times | 2 d |
| [S1c.2.5](s1c.2.5_the_report.md) | The report, and what it may claim | 2 d |

## Acceptance for the phase

- **One command regenerates the whole report** from the corpus on a machine
  with the pinned systems, and names every system it could not run.
- Every problem has an `.ein` encoding **and at least two non-Ein encodings**,
  each with recorded provenance — where the encoding came from and who
  adapted it.
- **Every problem's answer is confirmed by a system that is not Ein**, and the
  confirmation is checked back into the `.ein` file as an `:expect`, so it is
  re-checked by `ein test` on a machine with no solver installed.
- Timings are **processes, cold**, taken through
  [`utils/bench_env.sh`](../../../utils/bench_env.sh) exactly as
  [`e2e_baseline.py`](../../../utils/e2e_baseline.py) takes Ein's — "the same
  workloads as *processes*, which is what the milestone's targets mean".
- **A missing system is a reported cell, never a skipped one.**
- The report states where Ein loses, by how much, and why the author thinks so.

## Risks

- **Encoding bias is the entire validity of the exercise.** Whoever writes six
  encodings knows one of the six systems best, and an unwittingly clumsy
  Prolog program is indistinguishable in the table from a slow Prolog.
  Mitigation, in [S1c.2.1](s1c.2.1_problem_corpus.md): start from *published*
  encodings — Rosetta Code's [Zebra puzzle](https://rosettacode.org/wiki/Zebra_puzzle)
  and [N-queens](https://rosettacode.org/wiki/N-queens_problem), each system's
  own documentation examples — record provenance per file, and never tune a
  rival's encoding. An adapted published program with a link beats a
  hand-written one with a good intention.
- **Lean is not a solver, and a timing column will be read as if it were.**
  `decide` / `native_decide` over a finite domain is a decision procedure by
  brute force, and a hand-written proof is a different artefact altogether.
  Lean stays in the corpus for the encoding column; the report says so at the
  point where the number appears, not in a footnote.
- **Datalog may not be able to state the problem.** Pure Datalog has no
  choice: bottom-up deduction cannot *pick* a house for the Norwegian. Whether
  the puzzle is expressible at all — under Soufflé's `choice-domain`, as
  generate-and-test over an enumerated candidate relation, or not at all — is
  the finding, and it is the most interesting cell in the table, because
  **Datalog is the closest formal relative of Ein's saturator**. If Soufflé
  needs an extension to express the puzzle, that is direct evidence for
  [M1d](../../m1d_satisfiability/README.md)'s premise.
- **Tool availability is real work.** Measured on the dev machine (Manjaro,
  `core`/`extra`/`multilib`, 2026-08-20): `z3` 4.16.0 and `cvc4` 1.8 are
  installed, `swi-prolog` 10.0.2-2 is in the sync db, and `clingo`,
  `souffle`, `cvc5`, `minizinc` and `lean` are **not in the configured
  repositories at all**. Half the field needs AUR, upstream binaries or a
  build, and that is what makes [S1c.2.2](s1c.2.2_systems_and_install.md) a
  stage rather than a line in a README.
- **The repo's CVC4 submodule was not CVC5, and is gone.** `smt/CVC4`
  pointed at version 1.8 from 2021, kept as a scratch checkout after M3 was
  dropped, and never checked out by anything. CVC5 is a different program
  with a different name, and this benchmark uses **CVC5** — so the submodule
  had no consumer here, and M1a
  [S1a.10.5](../../m1a_rust/p1a.10_single_implementation/s1a.10.5_removal.md)
  deinitialised it rather than making every clone fetch it. The three
  hand-written `.smt` files it sat beside **stay**, and are named below.
  `smt/README.md` has the one command that re-adds it if a stage ever wants
  1.8.
- **Comparison invites integration.** Every time a table shows a solver
  winning by 100×, the next thought is "so call it". That is M3, it is
  dropped, and if the numbers argue otherwise the argument belongs in a
  followup with the drop note quoted in it — not in a stage of this phase.

## Cross-links

- [`docs/lib/02`](../../../docs/lib/02-solvers-csp-sat-smt.md) — the
  catalogue the system list is drawn from (§3 SMT, §4 CP/MIP, §5 ASP,
  §6 logic programming); [`03`](../../../docs/lib/03-theorem-proving-formal-methods.md)
  for Lean
- [`smt/`](../../../smt/) — `4-queens.smt`, `einstain-problem.smt`,
  `einstain-problem-minus-15.smt`: three of this corpus's encodings already
  exist, hand-written in 2021, and they are the reason the two example
  problems in the user's brief are the two problems that scratch directory
  happens to hold
- [`examples/README.md`](../../../examples/README.md) — the catalog
  convention the benchmark corpus follows
- [M5](../../m5_presentation/README.md) Track A/B — the consumer
- [F13](../../followups/f13_puzzles_beyond_zebra/ideas.md) — the *other*
  benchmark direction (BBH, ProofWriter, LogiQA): NL-shaped, M2-gated, and
  deliberately not this
