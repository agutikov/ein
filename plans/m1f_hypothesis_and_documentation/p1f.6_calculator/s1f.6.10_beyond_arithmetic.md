# S1f.6.10 — Investigation: what else the theory can do

**Phase:** [P1f.6](README.md)
**Status:** **done 2026-11-12, and shown to the user**, who decides what becomes a stage: the seven groups of
probes run, [`beyond_arithmetic.md`](beyond_arithmetic.md) written, sixteen probes banked under
[`examples/beyond_arith/`](../../../examples/beyond_arith/), three findings filed (E-29 to E-31), four decisions
taken by default (Q-M1f.186 to .189), eight questions put ([§ T1 record](#t1-record-2026-11-12) onward). No
stdlib module and no engine source moved. Planned 2026-11-08 as a report: probes and a page, no change to the
theory unless the user rules one.
**Depends on:** [S1f.6.8](s1f.6.8_rational_numbers.md) and [S1f.6.9](s1f.6.9_width.md). It works on the ruled design of
2026-11-11 ([the rulings](design.md#the-rulings-2026-11-11)): nested numbers of any length, so a bound in a probe is a
choice of the probe and no longer the theory's width.

## The question

The user: *"investigation of what can be done with this theory except simple arithmetic, can it be used for any
kind of theorems proving, or solving equations, or numbers theory exploration, or anything else"*.

Arithmetic itself is a circuit and never searches. **Inverse questions are where Ein's search comes in**:
given part of a computation, find the rest. The investigation asks what Ein makes of them, and what the
saturation-only theory gives besides.

## What it probes, each with a program, its answer and its cost

1. **Tables run backwards.** A table is a relation, not a function. A rule can read the full adder by sum and
   one operand, which is subtraction, and the digit-product table by product, which is a divisor. Probes:
   - how much of x + 4567 = 127645 saturation alone solves, column by column, with no hypothesis;
   - where a carry it cannot decide forces a guess.
2. **Equations by search.**
   - **Unknown digits as hypotheses.** An `hrule` proposes each unknown digit, the circuit checks, and R's
     states are the solutions.
   - **Cases:** x × 7 = 861, a linear equation with two unknowns at a narrow width, and an equation with no
     solution.
   - **For each:** R, the oracle's agreement, the enterings, and what an empty R's cores say about why there is
     no solution.
3. **Cryptarithms.** SEND + MORE = MONEY: letters as unknown digits, all different, the adder as the
   constraint. It is a Zebra-style puzzle on Ein's home ground ([idea 09](../../ideas/09-puzzles-beyond-zebra.md)),
   and the probe compares its cost with `zebra.ein`'s.
4. **Bounded theorem checking.**
   - **Identities at a width small enough to exhaust:** commutativity, associativity, distributivity, and the
     division identity X = Q·Y + R with R < Y. Every instance is computed and checked by a claim, so the result
     is a proof for that width, and the oracle can confirm it.
   - **The column lemma.** The full adder's 200 rows are checked once. Correctness for every width then follows
     by induction on columns, an induction made outside Ein. The probe states exactly which part Ein checks.
   - **What it cannot do:** quantify over unbounded numbers, or state an induction schema.
5. **Number theory exploration,** each bounded by a width or a range of requests:
   - divisors and primality by trial division;
   - gcd and lcm, from S1f.6.8's chain;
   - divisibility by digit sums, including casting out nines, checked over a width;
   - perfect numbers;
   - Collatz trajectories, bounded in steps.

   For each: what saturation can list, and what it costs in firings and rounds.
6. **Explanations.**
   - **Worked solutions:** a `:why` on each rule renders a school-style worked solution ("8 + 7 = 15: write 5,
     carry 1") ([idea 08](../../ideas/08-human-style-deductive-trace.md)).
   - **Proofs of no solution:** a refuted equation's core as a proof that it has none.
7. **Anything else the probes suggest,** named as a finding with its program.

## Deliverable

- `p1f.6_calculator/beyond_arithmetic.md`: each probe's program, answer and cost, each with the command that
  re-takes it.
- Probes banked under `examples/arith/`, each with a corpus entry where a claim can state its answer.
- Findings, each with a recommendation: what deserves a stage, what is out of reach, and why.
- Any wrong answer the probes find in the engine is a register row in `corpus/wrong_answers.toml` before
  anything fixes it.

## Acceptance

- The page, every claim on it carrying its command, date or test.
- The probes banked.
- The findings shown to the user, who decides what, if anything, becomes a stage.

## T1 record, 2026-11-12

**The probes.** All seven groups ran on the release binary, under `build/s1f.6.10/` (generators, runners, the
programs under `p/`, the read-outs under `out/`). The page is [`beyond_arithmetic.md`](beyond_arithmetic.md);
each number there carries its command. In brief:

| group | result |
|---|---|
| 1, tables backwards | x + 4567 = 127645 by saturation alone, 33 firings for three requests, no column needing a guess: read by an operand, the carry and the sum's digit the full adder has one row. Where a column has two answers (x × 4, x + x) saturation derives both walks and the next column ends the wrong one; the cost stays linear, 107 firings for 20 digits by 4. Two unknowns, x + y = S: every pair listed, S + 1 of them, at 10ⁿ partial walks |
| 2, equations by search | digits as hypotheses under `total-owed` and `functional`. x × 7 = 861: one state; `--single` takes 10n + 1 saturations with the constraint a column at a time and about 10ⁿ through `std.arith`; describing R is exponential in both, and at six digits had no answer in 300 s. 3x + 5y = 47: three states, as Python's. x × 7 = 862: R empty by owing, `Open`, the 28-set argument in the cores. The oracle agrees at 10 and at 20 candidates |
| 3, cryptarithms | SEND + MORE = MONEY solved and shown unique by `--single` in 17 747 saturations, 0.50 s, where `zebra.ein` takes 49; with its letters declared in the written order, no answer in 600 s (E-29); `ein test` cannot check its claim (E-30). TWO + TWO = FOUR: 7 states, as Python's |
| 4, bounded theorems | the six laws hold on every instance over 0 to 9, 4 100 instances, 271 266 firings, 0.42 s; commutativity over 0 to 99, 20 000; the division identity over 9 000. The same check with ordinary joins, 16.8 s. The column lemma: 200 rows of `fa` agree with addition by counting, 601 firings; Ein checks that part, and the induction on columns is outside it |
| 5, number theory | primes to 5 000, gcd and lcm to 31 digits, casting out nines to 9 999, perfect numbers to 250, Collatz to 837 799: every answer equal to Python's. Perfect numbers to 500 ended when the machine's 32 GB were used (E-26) |
| 6, explanations | a worked addition is the trace's `:why` sentences, with E-14 and E-17 standing between it and a pupil. A proof of no solution has three forms: silence, an owing R, or a dead root whose unsat core is the proof, x × 7 = 862 in three table rows |
| 7, what else | Pythagorean triples to 100 by a join on a number, 104 ordered triples; F(500), 105 digits, in 62 291 firings; Ein as a checker of certificates; and a cryptarithm with the tables read backwards as propagation, 639 nodes for 17 747 |

**No wrong answer of the engine was found**: every count agreed with Python and, where it reaches, with `ein
oracle`. No row of `corpus/wrong_answers.toml` was filed, and the register is unmoved: 305 fixed, 16 open and 1
not wrong (`grep '^status' corpus/wrong_answers.toml | sort | uniq -c`, 2026-11-12).

**The findings**, rows of [`findings.md`](findings.md):

- **E-29:** the obligation tree branches in the order the obligations are written, and that order decides
  whether a search ends.
- **E-30:** `ein test` decides a claim by describing R, where `--single` decides the same claim.
- **E-31:** describing R spends its time on a hypergraph transversal, 94 % of 5.1 s on the five-digit equation.

and E-3, E-18, E-23 and E-26 met again, each with a note there.

**What is banked**, under `examples/beyond_arith/`, sixteen programs and 24 claims (`ein test
examples/beyond_arith`): `inverse_addition`, `tables_backwards`, `equation_refuted`, `equation_by_search`,
`equation_two_unknowns`, `equation_no_solution`, `cryptarithm`, `laws`, `column_lemma`, `primes`, `gcd`,
`casting_out_nines`, `perfect`, `collatz`, `pythagorean`, `fibonacci`. `build/s1f.6.10/bank.py` wrote each with the
claim the engine printed, after a Python check of that claim. Each has a corpus entry. Not banked: SEND + MORE =
MONEY and every scale run, whose numbers are on the page.

**The decisions taken by default:**
[Q-M1f.186](../open_questions.md#q-m1f186--where-the-probes-of-s1f610-are-banked), the directory, beside
`examples/arith/` since `arith_search_free.rs` walks that one;
[Q-M1f.187](../open_questions.md#q-m1f187--which-probes-of-s1f610-are-banked-and-at-what-size), which probes and
at what size; [Q-M1f.188](../open_questions.md#q-m1f188--the-banked-probes-are-kept-by-hand), hand-kept, no
generator in the gate;
[Q-M1f.189](../open_questions.md#q-m1f189--a-users-program-calling-stdariths-tagged-calls), the banked chains
call the theory's tagged calls, and whether those are an interface is put to the user.

**The correction asked for:** [S1f.6.7](s1f.6.7_negative_numbers.md)'s record said a program importing `std.int`
pays 24.0 ms of load against 19.8; it now says 22.4 against 19.9, S1f.6.8's re-take, best of five, and that it
was re-taken.

**The goldens that move,** named before `EIN_BLESS=1`: `ein-cli/tests/golden/corpus_exits.txt`, 53 new cells,
the sixteen new files'; `ein-render/tests/golden/corpus_shapes.md5`, 672 new lines, their 42 renderings each;
and no other line of either (`git diff` on each, every added line naming `examples/beyond_arith/`). One test's list
was edited by hand, as S1f.6.7's and S1f.6.8's were: `ein-infer/tests/alive_invariant.rs`'s `SEEDED` gains
eleven of the programs, the `z` inside a nested number they write or the theory's own, and the `e` the three
inverse walks start their accumulator at. No engine source moved.

**Pages:** [`beyond_arithmetic.md`](beyond_arithmetic.md), new; `findings.md`; `open_questions.md`; the phase
README; `examples/README.md`, a row for the directory; `corpus/corpus.toml`, sixteen entries; `AGENTS.md`.

## Acceptance re-taken, 2026-11-12

- **The page, every claim on it carrying its command, date or test:** [`beyond_arithmetic.md`](beyond_arithmetic.md).
  The times of the search tables were taken while other runs of the stage shared the machine.
- **The probes banked:** sixteen, each with a corpus entry and a claim; the expensive ones under `build/` with
  their numbers on the page.
- **The findings shown to the user,** who decides what becomes a stage: the page's § What deserves a stage and
  its eight questions.
