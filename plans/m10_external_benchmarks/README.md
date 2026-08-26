# M10 — External benchmarks

**Estimate:** ~2.5 weeks — 5 stages, 12 days of stage estimates.
**Status:** **promoted to a milestone 2026-08-23** at the user's direction,
out of [M1c](../../docs/history/m1c_external_validation/README.md)'s P1c.2, where it was
created 2026-08-21. Nothing about the work changed: the five stages, their
estimates and their dependencies are as written, `S1c.2.<n>` became
`S10.<n>`, and the three questions about *method* moved with it as
[`Q-M10.1–3`](open_questions.md).
**Depends on:** [M1c P1c.1](../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance)
— not to run, but to *keep*: the answers this milestone establishes are
written back into the `.ein` files as `:expect`, and that form is P1c.1's.
Also on [M1a](../../docs/history/m1a_rust/README.md) having shipped its
[performance phase](../../docs/history/m1a_rust/README.md#p1a6--performance): a
comparison run against the pre-S1a.6 engine measures a version nobody ships.
**Blocks:** nothing on the critical path. [M5](../m5_presentation/README.md)
Track A is the consumer: its "head-to-head numbers where applicable" are this
milestone's output, and the paper is not where a benchmark should be run for
the first time.

---

## Why this exists: every check the repo has is relative

The conformance tiers compared two engines; the goldens compare ein.rs to its
own past; after [P1a.10](../../docs/history/m1a_rust/README.md#p1a10--one-implementation)
the second engine is gone, so what is left compares ein.rs to yesterday's
ein.rs. All of it answers *did this change?* None of it answers *is this
right?* — and the precedent is specific, recent and expensive:
`disjunctive-prune`'s `(neq ?h_other ?h1)` guard was wrong for a year, through
five phases of byte-exact parity, and what found it was an **independent
enumeration** of a puzzle's models written outside the engine on the day.

There are two ways to answer *is this right?* [M1c](../../docs/history/m1c_external_validation/README.md)
owns the first — an expectation written next to a rule and run by the engine.
This milestone owns the second: the same problem stated for six other systems.

**The two are one pipeline, which is why they were one milestone until
2026-08-23.** This milestone's ground truth lands in P1c.1's form: when Clingo
enumerates 32 models of `zebra2-minus-15` and Z3's blocking-clause loop
agrees, that answer is written into the `.ein` file as an `:expect`, and from
then on `ein test` re-checks it on a machine with no external solver installed
at all. **The external tools are needed to establish the answer, not to keep
it.**

> **The last clause needs a depth, measured 2026-08-26** by M1d
> [S1d.4.3](../../docs/history/m1d_satisfiability/the_vocabulary.md),
> which rewrote the same sentence in
> [M1c](../../docs/history/m1c_external_validation/README.md#splitting-them-did-not-split-the-pipeline).
> `ein test` exhausts, but at `--max-set-size 5`; `zebra2-minus-15`'s lattice
> ends at depth **22**, so at the default the re-check comes back **`NOT
> CHECKED`** — honest, and not a re-check. The honest promise is `ein test
> <file> -m 38`, which is a command somebody runs. **What this milestone owes
> its own acceptance bullet is therefore the depth as well as the answer**: an
> encoding whose `:expect` cannot be checked at the runner's default is checked
> in with the `-m` that checks it, or it is not checked in.

## Goal

**The same problem, stated once for each system, run by one harness, reported
in one table — answers first, times second.** Z3, CVC5, SWI-Prolog + CLP(FD),
Soufflé, Clingo and Lean 4 against `ein solve`, on a small corpus that every
one of them can express.

## Why a comparison, when M3 was dropped

M3 (SMT integration) was dropped 2026-08-18 and stays dropped: Ein has no
solver back-end, hands no `(hard-slice …)` to anybody, and this milestone
does not reopen that. The drop note already made the distinction — what survived
the milestone was [`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md)
"as external-tech catalogue and M5 Track A's *comparison* axis" — and the
sharpening it added is the reason to run the benchmarks at all:

> Ein never hands a slice to a solver … which sharpens the question rather
> than removing it: **the graph engine has to stand on its own numbers.**

There are no such numbers today. The engine's entire performance record
([baseline.md](../../docs/history/m1a_rust/measurements/baseline.md)) is Ein against
Ein: ein.rs against ein.py, against PyPy, against its own previous commit.
165× a Python implementation of the same algorithm says nothing about how the
algorithm compares to a CDCL solver on the same puzzle.

## Two products, and the first one is not the clock

1. **Answers.** Every system's answer for every problem, compared. For the
   under-determined entry that means the model *set*: Clingo's `--models 0`,
   a Z3 blocking-clause loop and a Prolog `findall` each enumerate
   independently, and if they agree, the count is established by three
   systems that share no code with Ein and no code with each other. This is
   the [`disjunctive-prune`](../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance) check
   made permanent: that bug survived a year of two engines agreeing and died
   to one independent enumeration.
2. **Times, and what had to be said to get them.** Wall clock as a process,
   peak RSS, and the encoding column — how many lines, and *what the language
   made the author state*. On n-queens that column is the interesting one:
   the kernel has no arithmetic ([Q17](../open_questions.md#q17--spatial-relation-formalisation)
   — "no integer-arithmetic position lattice"), so diagonals arrive as an
   extensional relation the way `right-of` does, while Z3 writes
   `(!= q1 (+ q2 1))` and is done. That is a real difference between the
   systems and the table should show it rather than hide it in a ratio.

## Stages

| stage | title | est. |
|---|---|---|
| [S10.1](s10.1_problem_corpus.md) | The problem corpus, and what a fair encoding is | 3 d |
| [S10.2](s10.2_systems_and_install.md) | The systems: versions, and how they install | 2 d |
| [S10.3](s10.3_the_runner.md) | The runner | 3 d |
| [S10.4](s10.4_answers_not_only_times.md) | Answers, not only times | 2 d |
| [S10.5](s10.5_the_report.md) | The report, and what it may claim | 2 d |

## Acceptance for the milestone

- **One command regenerates the whole report** from the corpus on a machine
  with the pinned systems, and names every system it could not run.
- Every problem has an `.ein` encoding **and at least two non-Ein encodings**,
  each with recorded provenance — where the encoding came from and who
  adapted it.
- **Every problem's answer is confirmed by a system that is not Ein**, and the
  confirmation is checked back into the `.ein` file as an `:expect`, so it is
  re-checked by `ein test` on a machine with no solver installed — **at a depth
  that reaches it**. `ein test` runs `-m 5`; a claim that needs more is checked
  in with the `-m` that checks it, or it is not checked in at all, because
  `NOT CHECKED` is not a re-check (M1d
  [S1d.4.3](../../docs/history/m1d_satisfiability/the_vocabulary.md)).
- Timings are **processes, cold**, taken through
  [`utils/bench_env.sh`](../../utils/bench_env.sh) exactly as
  [`e2e_baseline.py`](../../utils/e2e_baseline.py) takes Ein's — "the same
  workloads as *processes*, which is what M1a's targets meant".
- **A missing system is a reported cell, never a skipped one.**
- The report states where Ein loses, by how much, and why the author thinks so.

## Risks

- **Encoding bias is the entire validity of the exercise.** Whoever writes six
  encodings knows one of the six systems best, and an unwittingly clumsy
  Prolog program is indistinguishable in the table from a slow Prolog.
  Mitigation, in [S10.1](s10.1_problem_corpus.md): start from *published*
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
  [M1d](../../docs/history/m1d_satisfiability/README.md)'s premise.
- **Tool availability is real work.** Measured on the dev machine (Manjaro,
  `core`/`extra`/`multilib`, 2026-08-20): `z3` 4.16.0 and `cvc4` 1.8 are
  installed, `swi-prolog` 10.0.2-2 is in the sync db, and `clingo`,
  `souffle`, `cvc5`, `minizinc` and `lean` are **not in the configured
  repositories at all**. Half the field needs AUR, upstream binaries or a
  build, and that is what makes [S10.2](s10.2_systems_and_install.md) a
  stage rather than a line in a README.
- **The repo's CVC4 submodule was not CVC5, and is gone.** `smt/CVC4`
  pointed at version 1.8 from 2021, kept as a scratch checkout after M3 was
  dropped, and never checked out by anything. CVC5 is a different program
  with a different name, and this benchmark uses **CVC5** — so the submodule
  had no consumer here, and M1a
  [S1a.10.5](../../docs/history/m1a_rust/README.md#s1a105--the-removal)
  deinitialised it rather than making every clone fetch it. The three
  hand-written `.smt` files it sat beside **stay**, and are named below.
  `smt/README.md` has the one command that re-adds it if a stage ever wants
  1.8.
- **Comparison invites integration.** Every time a table shows a solver
  winning by 100×, the next thought is "so call it". That is M3, it is
  dropped, and if the numbers argue otherwise the argument belongs in a
  followup with the drop note quoted in it — not in a stage of this milestone.

## Non-goals

- **A solver back-end.** M3 (SMT integration) was dropped 2026-08-18 and stays
  dropped — see [`plans/README.md`](../README.md) § Roadmap. Ein never hands a
  slice to Z3. Comparing against a solver is the opposite of integrating one,
  and the drop note already says the comparison axis survives.
- **A benchmark *suite*.** This corpus is small, curated and cross-language by
  construction; the sweep over reasoning benchmarks — BBH `logical_deduction`,
  ProofWriter, LogiQA — is [F13](../followups/f13_puzzles_beyond_zebra/ideas.md)'s
  territory and needs [M2](../m2_nl_to_ir/README.md)'s NL frontend to be
  interesting.
- **Ein's own performance work.** [P1a.6](../../docs/history/m1a_rust/README.md#p1a6--performance)
  owns that and closed on its targets. A comparison that shows Ein slow
  somewhere is a *finding*; acting on it is a milestone somewhere else.
- **Winning.** A benchmark built to be won gets tuned until it is. The
  deliverable is a table with its caveats — including the rows where a CP-SAT
  solver beats the graph engine by orders of magnitude on a problem that is
  all arithmetic, which is the expected result on n-queens and is not a
  defect.
- **New reasoning features.** If a comparison shows that a problem cannot be
  *stated* in ein-lang, that is a finding for
  [M1d](../../docs/history/m1d_satisfiability/README.md) or a followup, not a stage here.

## Open questions

[`open_questions.md`](open_questions.md) — `Q-M10.<n>`, all three arriving
2026-08-23 with the promotion, where they were `Q-M1c.3–5`: what makes an
encoding fair, whether a proof assistant belongs in a timing table, and where
the harness lives.

## Cross-links

- [`docs/lib/02`](../../docs/lib/02-solvers-csp-sat-smt.md) — the
  catalogue the system list is drawn from (§3 SMT, §4 CP/MIP, §5 ASP,
  §6 logic programming); [`03`](../../docs/lib/03-theorem-proving-formal-methods.md)
  for Lean
- [`smt/`](../../smt) — `4-queens.smt`, `einstain-problem.smt`,
  `einstain-problem-minus-15.smt`: three of this corpus's encodings already
  exist, hand-written in 2021, and they are the reason the two example
  problems in the user's brief are the two problems that scratch directory
  happens to hold
- [`examples/README.md`](../../examples/README.md) — the catalog
  convention the benchmark corpus follows
- [M1c](../../docs/history/m1c_external_validation/README.md) — the milestone this was a
  phase of until 2026-08-23; [P1c.1](../../docs/history/m1c_external_validation/README.md#p1c1--stdlib-conformance)
  owns the `:expect` form every answer here is checked back into
- [M5](../m5_presentation/README.md) Track A/B — the consumer
- [F13](../followups/f13_puzzles_beyond_zebra/ideas.md) — the *other*
  benchmark direction (BBH, ProofWriter, LogiQA): NL-shaped, M2-gated, and
  deliberately not this
