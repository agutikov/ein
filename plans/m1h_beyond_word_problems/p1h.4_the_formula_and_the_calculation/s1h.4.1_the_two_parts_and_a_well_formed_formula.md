# S1h.4.1 — The two parts on the banked problems, and *well-formed* as a conjunction of the checks that exist

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **Read-only on the engine:** it runs the binary on banked and
scratch programs and writes a page; no source under `ein.rs/` and nothing outside `plans/` and `build/`
moves.
**Depends on:** the word problems as banked under
[`examples/word_problems/`](../../../examples/word_problems/README.md); the calculator as shipped, with
[S1f.6.23](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.23_the_loaders_arity_check.md),
[S1f.6.12](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.12_malformed_numbers.md)'s build and
[S1f.6.24](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.24_division_by_zero_at_the_call.md).

## What it rests on

The user, 2026-11-12 (reply 36):

> How we can divide 2 parts? inference of a formula and calculations, the second is already solved (in
> arithmetic at least), but only if formula is well-formed, correct as a formula itself (all operations have
> required arguments, numbers and names correctly used, no division by zero, etc...). Yes, this is a phase in
> M1h

The milestone README: *The checks of a formula the reply lists are, each, something an earlier stage built or
ruled … The phase states the condition "the formula is well-formed" as their conjunction and shows what is
still missing from it.*

## What it produces

The first section of `formula_and_calculation.md` in this directory: the reply's first question, *how we can
divide 2 parts*, answered on the banked problems as they run today; and the condition under which the second
part *"is already solved"*, written as a table of checks with a failing program for each.

## Tasks

1. **T1h.4.1.1 — the two parts of a run.** On `bob1`, `boxes12`, `pencils`, `more` and `contra`: every step
   of `ein solve FILE -G --trace T.md` put on one side of the line by the module that declares its rule (the
   three modules of `examples/word_problems/`, or the calculator's), counted, with the first and last step of
   each call into the calculator. Re-takes the phase's figure for `boxes12` (145 steps, 12 and 133, on
   2026-11-13) and gives the same for the other four. Where the line is not sharp, the page says so with the
   step: a word rule that reads the calculator's answer (`together-is-the-sum`) has premises on both sides.
2. **T1h.4.1.2 — where the formula is today.** For the same problems, from `ein saturate FILE --dump`: every
   request to the calculator with the rule that asked, and the `(needs Q Q')` facts of the plan. Written out
   beside the formula a pupil would write for the problem, *(5 + 7) ÷ 3*: what of that formula the state
   holds (the operations, each as a request; the operands of each, as numbers already computed), and what it
   does not (that the dividend *is* the sum: the nesting). This is the milestone README's sentence,
   *Today the formula of a word problem exists nowhere: it is the order in which rules ask the calculator*,
   shown on a dump.
3. **T1h.4.1.3 — the checks, each with a failing program.** One row for each clause of the reply, each a
   scratch program under `build/s1h.4.1/` whose formula fails that clause, run on the day's binary, with
   what the run says and its exit code:
   - *all operations have required arguments*: a request with an argument too few. The loader's refusal
     (S1f.6.23);
   - *numbers … correctly used*: a malformed number as an operand. `(malformed REQUEST)`, and root dead
     ([`stdlib/README.md` § Malformed requests](../../../stdlib/README.md#malformed-requests-what-is-flagged-and-what-stays-silent-m1f-s1f612));
   - *names correctly used*: an operand that names nothing, `(undeclared NAME REQUEST)`; a name stated
     twice; and the two the user sent to P1g.10 with the expression layer, a used name nothing states and a
     name defined through itself, each marked *built* or *not built* on the day;
   - *no division by zero*: the flag at the call and `(false)` (S1f.6.24);
   - *etc.*: see T4.
   For each row: **who checks it** (the loader, a rule of the calculator, nothing), **when** (at load, at the
   request), and **what the word theory's own problem looks like when it fails** (a word problem that leads
   to a division by zero, a share among no classes, run as a scratch problem over the banked theory).
4. **T1h.4.1.4 — what is still missing from the conjunction.** The reply ends *"etc..."*. The stage lists
   what a formula can get wrong that no row of T3 catches, each with a program and today's silent or wrong
   outcome, and claims no more than it shows. The candidates to try, none assumed: a request whose operand
   never gets a value, so that nothing is computed and nothing says so; operands of two kinds of thing,
   apples added to pears, which the word theory avoids by a rule's shape and no check states; a number of
   the wrong kind for the operation, a natural where an integer is expected, which is
   [M1g's P1g.8](../../m1g_logic/p1g.8_a_theory_of_types/README.md)'s. Each is a row with an owner or the
   word *none*.
5. **T1h.4.1.5 — the line.** One paragraph and one table: what the calculation may assume of what it is
   handed, what it does today when the assumption fails, clause by clause; and so what *inference of a
   formula* has to deliver for the second part to be *already solved*.
6. **T1h.4.1.6 — the page.**

## Acceptance

- The five problems are on the page with their steps counted on each side of the line, by a command.
- Each clause of the reply has a row with a failing program, its output and its exit code, taken on the
  day's binary.
- The list of what is missing has a program for every item it claims.
- Nothing outside `plans/` and `build/` moved.

## What it must not do

- **Not add a check.** A missing one is a row of the table and, where it is the engine's or the
  calculator's, a row of `findings.md` with an owner.
- **Not change a rule of the calculator or of the word theory.**
- **Not define the formula's data.** That is [S1h.4.2](s1h.4.2_the_formula_as_an_object.md)'s, after
  P1g.10's.

## The standing rules

A wrong answer of the engine met by a scratch program is a row in `corpus/wrong_answers.toml`, filed `open`
with its witness before anything fixes it. Findings go to `findings.md` in this directory, opened at the
first; questions to [`open_questions.md`](../open_questions.md) under the next free id. Scratch stays under
`build/s1h.4.1/`. The link check is run before the commit, which changes pages only.
