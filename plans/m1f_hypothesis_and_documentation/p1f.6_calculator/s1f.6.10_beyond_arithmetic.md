# S1f.6.10 — Investigation: what else the theory can do

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-08. A report: probes and a page, no change to the theory unless the user rules
one.
**Depends on:** [S1f.6.8](s1f.6.8_rational_numbers.md) and [S1f.6.9](s1f.6.9_width.md).

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
