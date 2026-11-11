# S1f.6.17 — Binary logic: truth tables, gates, and what they are good for

**Phase:** [P1f.6](README.md)
**Status:** planned 2026-11-11, on the user's instruction of that day. Not started.
**Depends on:** [S1f.6.16](s1f.6.16_binary_arithmetic.md), for the two digit names and the binary adder it
rebuilds from gates.

## The instruction

The user, 2026-11-11: *"binary logic, truth tables, can be there any applications for it?"*

## What it is

The calculator's tables are truth tables already: a full adder's eight rows are a function of three bits. This
stage goes one level down, to the gates the tables are made of, and asks what Ein does with them.

- **The gates as tables of facts:** `not`'s two rows, and the four rows each of the two-input functions. The
  names are the stage's to choose with care: `and`, `or`, `not`, `eq` and `false` are the engine's own words,
  and the value rule keeps `and` and `or` from ever heading a value, so a gate is not called `and`.
- **A circuit as a program:** wires are names, a gate instance is a fact, and one rule per gate type
  propagates values. Saturation is then circuit simulation, search-free, like the calculator.
- **The binary full adder built from gates,** and its eight outputs held equal to S1f.6.16's table: the
  calculator's own gate, derived instead of written.

## The applications, each tried and each answered with a program or a *no*

1. **Simulation:** inputs given, outputs derived. No search.
2. **The truth table of a circuit.** The inputs are **hypotheses**: one candidate per input and value, the
   theory saying an input has exactly one value. Then R is every input assignment, a query's projection
   classes are the table's rows, and `ein solve` prints the truth table. This is where search belongs, and it
   is Ein's existing answer shape used as it is.
3. **Satisfiability:** is there an input for which the output is 1? The same program with the output
   required: R is the satisfying assignments, and empty means unsatisfiable, with the cores saying why. A
   small SAT solver falls out of the engine, and the stage measures how small *small* is: the commitments
   limit, and what the region costs at 8, 16 and 24 inputs.
4. **Equivalence of two circuits:** the gate-built adder against the table, or two forms of one function.
   Equivalent when no input tells them apart, which is an unsatisfiability.
5. **Puzzles of truth and lies** (knights and knaves,
   [idea 09](../../ideas/09-puzzles-beyond-zebra.md)): a statement is a circuit over who is truthful.
6. **Synthesis:** which gate, put here, gives this table? The gate's type as the hypothesis. Tried on a
   one-gate hole and reported.

## Tasks

1. **T1, a prototype and a page, shown to the user:** each application above as a real program with its
   computed answer and every artefact, the names proposed, and what did not work.
2. **What ships,** as ruled: a small module of gate tables and propagation rules, its test programs, and the
   examples.
3. **The findings,** to [`findings.md`](findings.md). Expected, and to be confirmed: the commitments limit on
   a truth table of many inputs; and that an input's *exactly one value* wants a typed obligation.

## Acceptance

- Each application has a program and its computed answer, or a stated reason it has none.
- The gate-built full adder equals the table, held by a test.
- The page says which applications are search-free and which are a search, and what each costs.

## Where it leads

Binary logic is propositional logic's semantics: a formula is a circuit, a valuation an input assignment, a
tautology a circuit that is 1 on every row. Formulas as objects, proof systems, first-order logic and Peano
arithmetic are not this stage's and not this phase's; the coordinator proposed them as a milestone of their
own on 2026-11-11, and this stage is its starting point.
