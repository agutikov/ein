# S1c.2.1 — The problem corpus, and what a fair encoding is

**Phase:** P1c.2 (External benchmarks)
**Estimate:** 3 days

## Context

Four problems to start, and one rule per encoding that decides whether the
whole table means anything.

| problem | why it is in | ein encoding |
|---|---|---|
| `zebra` | the canonical statement, unique model — Ein's home turf and the one every system has a published encoding of | [`examples/zebra.ein`](../../../examples/zebra.ein) |
| `zebra2` | the unified is-a / `*-loc` variant the engine is actually measured on | [`examples/zebra2.ein`](../../../examples/zebra2.ein) |
| `zebra2-minus-15` | **32 models** — the entry where *counting* separates the systems, and [M1d](../../m1d_satisfiability/README.md)'s subject | [`examples/zebra2-minus-15.ein`](../../../examples/zebra2-minus-15.ein) |
| `n-queens` (n = 4, 6, 8) | deliberately **not** Ein's home turf: arithmetic-shaped constraints, a parameterised family, no natural-language provenance | has to be written |

Three of the four already have a non-Ein encoding in the repo:
[`smt/einstain-problem.smt`](../../../smt/einstain-problem.smt),
[`einstain-problem-minus-15.smt`](../../../smt/einstain-problem-minus-15.smt)
and [`4-queens.smt`](../../../smt/4-queens.smt), hand-written in 2021 and kept
after M3 was dropped as "encoding examples — what the Zebra puzzle looks like
when written for a solver rather than for the graph engine". They are the
starting point and they are also a warning: they are one person's SMT-LIB, not
a published encoding, and this stage's rules exist to stop the rest of the
corpus being written the same way.

## The fairness rules

The stage's real deliverable, written into `bench/README.md`:

1. **Provenance per file.** Where the encoding came from — a URL, a citation,
   or "written here, from the problem statement" — and who adapted it. An
   adapted published program with a link beats a hand-written one with a good
   intention.
2. **Idiomatic per system, not transliterated.** The Prolog entry is a CLP(FD)
   program the way a Prolog programmer writes one, not the ein rules rendered
   as clauses. Transliteration measures the translator.
3. **No tuning against the clock.** The first working idiomatic version is the
   one that is timed. If a faster version is found later, both stay in the
   corpus and the report says which one the number came from.
4. **Every encoding answers the same question** — it prints a *model*, not
   `sat`. A system that reports satisfiability without an assignment has
   answered a weaker question and its cell says so.
5. **The answer's printed form is declared** with the encoding, because
   [S1c.2.4](s1c.2.4_answers_not_only_times.md) has to parse it into a
   canonical form.

## n-queens in ein-lang is the interesting part

The kernel has **no arithmetic** — [Q17](../../open_questions.md#q17--spatial-relation-formalisation)
settled that spatial structure stays IR-native, "declarative `square-fwd` /
`square-bwd` rules + property facts; no integer-arithmetic position lattice",
and `right-of` / `next-to` in the zebra corpus are extensional relations over
`House × House` because of it. So a diagonal is not `|qi − qj| ≠ |i − j|`; it
is an enumerated `attacks` relation, generated per board size the way
[`examples/gen_zebra2_variants.py`](../../../examples/gen_zebra2_variants.py)
generates its variants.

Two consequences, and the second is a rule:

- The encoding **grows with the board** where every rival's is constant-size.
  That is a real property of the language and belongs in the report's encoding
  column, not in a footnote apologising for it.
- **The generator may compute the board; it may never compute the solution.**
  Enumerating which squares attack which is the arithmetic the kernel declines
  to do. Enumerating which placements are consistent is *solving*, and a
  benchmark whose input is pre-solved is a fraud. The line is bright and the
  stage writes it down.

If the resulting encoding turns out degenerate — if `attacks` is so large that
the puzzle is trivially propagated, or so large that loading dominates the
solve — that is a finding about the language and is reported as one. It is not
a reason to quietly drop n-queens: the user named it, and a problem the engine
is bad at is the only kind that tests the claim.

## Acceptance

- `bench/corpus.toml` — one entry per (problem, system) with the file, the
  provenance, the invocation, and the expected answer shape. A file with no
  entry fails a completeness check, the way
  [`conformance/corpus.toml`](../../../conformance/corpus.toml) already works
  for `.ein`.
- Every problem has **an ein encoding and at least two non-Ein encodings**.
- `bench/README.md` carries the five fairness rules and the per-problem
  catalog, in [`examples/README.md`](../../../examples/README.md)'s style —
  one line per file.
- The n-queens ein encoding exists for at least n = 4, 6, 8, its generator is
  checked in, and the generator's output is checked in too (a corpus that
  needs a script to exist is a corpus that rots).
- **What is deliberately not in the corpus is written down** with the reason.

## Tasks

### Task T1c.2.1.1 — The manifest and the catalog
### Task T1c.2.1.2 — The zebra family, per system

Start from published encodings: Rosetta Code's
[Zebra puzzle](https://rosettacode.org/wiki/Zebra_puzzle) carries entries in
dozens of languages, and each system's own documentation carries puzzle
examples. Take what exists, adapt it, cite it, and record what was changed and
why — the first task is a survey of what is already published per system, not
a writing task.

### Task T1c.2.1.3 — n-queens: the generator, the ein encoding, the rivals

[Rosetta Code N-queens](https://rosettacode.org/wiki/N-queens_problem) for the
rivals; the generator + `attacks` relation for ein. `smt/4-queens.smt` is the
SMT entry at n = 4 and shows what the same constraint costs in a language that
has `+`.

### Task T1c.2.1.4 — The fairness rules, written down
### Task T1c.2.1.5 — What is not in the corpus

Sudoku, graph colouring, SEND+MORE, pigeonhole. Each is a candidate; none is
added until it asks a question the four above do not. Record the reasoning so
the next person does not re-litigate it.

## Notes

- The corpus grows **only** when a problem asks a new question. Four problems
  × six systems is already 24 encodings to keep honest, and the cost of one
  more row is paid six times.
- `zebra2-minus-15` is the entry that justifies the whole exercise: it is the
  one where Ein currently *cannot finish*, and where three independent
  enumerations settle what the right answer is before
  [M1d](../../m1d_satisfiability/README.md) starts arguing about how to get
  there.
