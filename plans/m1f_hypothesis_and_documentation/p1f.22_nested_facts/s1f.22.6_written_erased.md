# S1f.22.6 — `written` erased; a numeral is checked by the operation that takes it

**Phase:** [P1f.22](README.md)
**Status:** created and **started 2026-11-18** on the user's word (below). It runs before
[S1f.22.5](s1f.22.5_save_and_load.md)'s tasks 2 to 4, whose work in progress is paused on the branch
`s1f.22.5-save-load` (`db562a86`) on the same word; S1f.22.5 resumes when this stage is done and the multi-context
drawings are drawn again.
**Ruling:** [Q-M1f.404](../open_questions.md#q-m1f404--written-is-erased-a-numeral-is-checked-by-the-operation-that-takes-it).

## The instruction

The user, 2026-11-18, verbatim, after asking of `written` *"WTF is written ctx???"* and *"When it was added and what
for?"*, and being told it is Q-M1f.272's loader step (E1, `f555c7e0`, 2026-11-14), made for Q-M1f.210:

> revert this decision, completely erase this `written` crutch. insert malformed numbers detection into decimal and
> binary arithmetic operations. all numbers taken from input program will be checked once and labeled valid, and
> numbres produced by operations has to be labelled valid already. put current running stage into branch and pause
> its work, first remove `written` and then again draw multi-context examples and continue p1f.22.5

## What it reverts

- **Q-M1f.272** (a representation module reads what a file writes through `written`), **Q-M1f.275** (`written`
  states every fact a file nests), **Q-M1f.279** (a written fact is derived by the loader's step) and
  **Q-M1f.280** (a program does not write `written`): erased. Their decision files go, and every place that cites
  one cites Q-M1f.404 instead (an id is kept and never reused; a superseded id still cited is rewritten to the id
  that carries the decision).
- **Q-M1f.210**, *flagged where it is written, with no calculation reading it*: that half is reverted. A malformed
  number is found when an operation takes it. **Q-M1f.276**, the check of an integer's and a rational's own shape
  over `written`: the same.
- What other decisions and pages say of `written`: Q-M1f.391's *the loader's written step reads through every
  negation*, Q-M1f.392's cause, the reasoning's pages that call `written` *a context the loader makes*, the
  reserved name and its refusal (`07-diagnostics.md` § 2.30), the KB view's `written` octagons and the `written` in
  a frame's label (finding N-23, which this ends).

## The design the ruling gives, and the readings taken

The ruling's sentences, and how this stage reads each. Each reading is a default, put as a question
(Q-M1f.405 onward) with its alternatives, and stands until the user revises it.

1. **"all numbers taken from input program will be checked once and labeled valid"**: a numeral an operation
   takes as an operand, and that no operation produced, is checked by the representation's rules and gets the
   label *valid*, or is flagged malformed, and the flag derives `(false)` as today. *Once*: the label is a fact,
   so each numeral, each place of it, is checked once whatever reads it.
2. **"numbers produced by operations has to be labelled valid already"**: an operation labels its result valid as
   it produces it, and a result is never checked.
3. **A numeral no operation takes is not checked.** Examples are a numeral in a `let` nobody asks about, in a fact
   of the program's own relation, or in a rule's conclusion that never fires. This is the consequence of putting
   the detection into the operations. The programs that hold such a numeral to be flagged
   (`tests/stdlib/math/decimal/02_in_a_let_nobody_asks_about.ein`, `03_in_a_fact_of_the_programs_own.ein`,
   `04_in_a_rules_conclusion.ein`, and binary's twins) are changed to say that it is not, or deleted.
4. **Where the rules live**: what a well-formed numeral is, and the label, stay the representation's
   (`std.math.decimal`, `std.math.binary`), as Q-M1f.210 placed them. The operations of `std.math.decimal-arith`
   and `std.math.binary-arith` ask for the check of an operand and label their results.
5. **`std.math.int` and `std.math.rat`**: the ruling names decimal and binary. Their checks of an integer's or a
   rational's own shape also read `written`, and erasing it means they move too. By the same principle, they move
   into the operations that take an integer or a rational, with the same label.
6. **The standing rule *no rule of the calculator joins two facts on a number*** stands. The design keeps it,
   for instance by checking an operand where its request is read and carrying the label with the request's tag
   (an operand's fault travels with its tag, Q-M1f.206). If no design keeps it, the stage stops and puts that to
   the user.

## Tasks

1. **The census and the design**, short, in this file's Record. The census covers every place `written` is:
   engine, standard library, `tests/`, `examples/`, `corpus/`, `docs/`, decisions, `utils/`, `AGENTS.md`. The
   design covers the label's form and how each operation of each base reads and writes it. It is shown on real
   programs: a well-formed numeral, a malformed one an operation takes, a malformed one nobody takes, and a result
   fed to a second operation. The readings above go into `open_questions.md` as Q-M1f.405 onward, each with a
   **Default**.
2. **The engine**: the loader's `written` step, the reserved name and its refusal, the internal rule name
   `#written`, and every reader. That includes the KB view's `written` octagons, the state writer's leaving out of
   `written`, the alive invariant's banked lists, and the narration.
3. **The standard library**:
   - the four modules' checks, and the operations of both bases and of `int` and `rat`;
   - the closing rules' lists, with their tests (`arith_theory.rs`, `int_theory.rs`, `rat_theory.rs`,
     `num9_theory.rs`, `binary_theory.rs`);
   - `MANIFEST.sha256`, and the `04-stdlib/` rule pages through their generator;
   - the programs under `tests/stdlib/`, the mutation survivors, and the census.
4. **The pages and the decisions, in the same commits.** `AGENTS.md`'s lines on `written` are edited in place.
5. **Verification**, once, on the last commit:
   - `./run_tests.sh`;
   - the calculator's batch by hand, `cargo test -p ein-infer --features counters --test arith_batch`;
   - the nightly, since this is a large engine and standard-library change;
   - `utils/ci_container.sh`, since the KB view's DOT writer changes.

## Acceptance

- No relation `written` is left in the project. `git grep -n -w written` is shown with every line it still prints,
  each of them the English word.
- Every corpus program's answer is as before, except the programs whose malformed numeral no operation takes
  (reading 3). Each of those is named, with its answer before and after.
- `./run_tests.sh`, the nightly, the batch and `utils/ci_container.sh` exit 0 on the last commit.

## After it

The multi-context drawings in [`diagrams/`](diagrams/) (`render-held-in-two-contexts`,
`render-two-contexts-of-one-relation`, and S1f.22.4's `render-two-contexts`) are drawn again. Then S1f.22.5 resumes:
its branch merges master and continues, in a worktree, and merges back into master.

## Record

Not started.
