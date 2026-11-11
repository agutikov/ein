# S1g.2.4 — Applications and scale: puzzles of truth and lies, Ein's own three states as a third truth value, and how many letters is too many

**Phase:** [P1g.2](README.md)
**Status:** planned 2026-11-11, not started. A report with banked programs.
**Depends on:** [S1g.2.2](s1g.2.2_valuations_as_hypotheses.md).
**Pulls in:** [idea 09](../../ideas/09-puzzles-beyond-zebra.md)'s knights and knaves;
S1f.6.17's application 5 (puzzles of truth and lies) and its measurement at 8, 16 and 24 inputs, restated for
formulas; [`absent_semantics.md`](../../../docs/kernel/inference/absent_semantics.md)'s three-state model.

## What it asks

*"can be there any applications for it?"* (the user, 2026-11-11, of binary logic). For formulas: what a
program that imports the propositional theory can do that a zebra program cannot, where the theory stops
being practical, and what Ein's own notion of truth has to do with the two-valued one.

## The four parts

**1. Puzzles of truth and lies.** Smullyan's island: a knight's statements are true, a knave's false. A
statement is a formula over letters *X is a knight*; that A said F is the formula `(Iff knight-A F)`; the
puzzle is a set of required formulas, and R is its solutions. A sketch, to be replaced by programs whose
answers are computed. It rests on S1g.2.2's `must-hold` and an `Iff` table; joined to them it ran once:
k_s 1, and the claim held (`ein test`, release binary at `8860d616`, 2026-11-11).

```lisp
;; sketch. A says: "we are both knaves." What are A and B?
(atom knight-A) (atom knight-B)
(eval      (Iff knight-A (And (Not knight-A) (Not knight-B))))
(must-hold (Iff knight-A (And (Not knight-A) (Not knight-B))))
(solve :hrules (guess (knight-A 0) (knight-A 1) (knight-B 0) (knight-B 1)))
(query :goal (val ?who ?v)
       :expect (model (val knight-A 0) (val knight-B 1)))   ; A is a knave, B a knight
```

To be shown on three or four puzzles of rising difficulty, one with no solution and one with several, and
compared with the same puzzle written directly as Ein rules with no formula objects: which is shorter, which
explains itself better in `--trace`, and what the cores say of the unsolvable one. A statement about a
statement (*A says that B would say…*) nests an `Iff` and stays propositional; a statement about itself does
not, and is [S1g.5.4](../p1g.5_peano_and_induction/s1g.5.4_goedel_made_concrete.md)'s.

**2. A clue as a formula.** Whether a Zebra clue is usefully a formula over ground atoms, *the Englishman
lives in the red house* as a big disjunction over houses, and what that shows about why the puzzle is not
written that way: its rules quantify, and a propositional encoding grounds them. One clue, grounded, with the
count of letters. This is the bridge to [P1g.4](../p1g.4_first_order_logic/README.md).

**3. Three values.** Ein's store has three states for a fact P: asserted, negated (a stored `(not P)`), and
neither, which `(unknown P)` reads. That is not two-valued logic. The stage asks, on programs:

- whether evaluating a formula under a **partial** valuation, a letter with no value, is the strong Kleene
  logic K3 when the tables are extended with a third value `u`; the tables are a few rows more, and the
  check is against a Rust K3 evaluator;
- whether a partial valuation as a state of R (drop the obligation that every letter has a value) gives the
  smaller answers [S1g.2.2](s1g.2.2_valuations_as_hypotheses.md) noticed are missing: then R holds every
  consistent partial valuation, 3ⁿ of them, and its **minimal line** is the minimal satisfying partial
  valuations, the prime implicants. **To be tried;** the coordinator does not know that the minimal line is
  that.
- what the same construction says of Ein itself: a state of a puzzle is a partial valuation of its ground
  atoms, and S1g.4.1 uses the reading.

**4. Scale.** A table, each cell a command: letters n against k_s, saturations, time and peak memory, for a
tautology, a satisfiable requirement with one model, and an unsatisfiable one, with `ein solve` describing R,
with `--single`, with `-n 1`, and where the region limit and budget end each. Then the same for formulas of
growing size at fixed n. The expectation is plain: describing R is 2ⁿ and ends in the low tens of letters;
finding one model may go further. The page says what the numbers are and does not present the theory as a
SAT solver. The pigeonhole formulas, which are hard for resolution, are the unsatisfiable family, and their
cores are shown.

## Tasks

1. **T1g.2.4.1 — the puzzles,** banked under the examples' logic directory with corpus entries and claims.
2. **T1g.2.4.2 — the grounded clue,** one page.
3. **T1g.2.4.3 — three values:** the K3 tables, the Rust check, the partial-valuation program and its
   minimal line.
4. **T1g.2.4.4 — the scale table,** run through `utils/bench_env.sh`, with the commands.
5. **T1g.2.4.5 — the page,** `applications.md` in this directory: which applications are search-free, which
   are a search, what each costs, and which the stage answers with a *no*.

## Acceptance

- Each puzzle has a program, a claim `ein test` holds, and the oracle's agreement where it reaches.
- The three-valued tables agree with the Rust K3 evaluator on every formula up to the bound.
- The scale table's every number has its command and date.
- The page names what was tried and did not work.

## What it may find

- **The commitments limit, first.** S1f.6.17 expects it on a truth table of many inputs; this stage measures
  the same wall for formulas.
- **A wrong answer past the limit.** Programs here are monotone, so the state route should stay right; a
  disagreement with the oracle is a register row.
- **That partial valuations are the better default.** Then S1g.2.2's module is re-shaped, and that goes to
  the user with the two side by side.
