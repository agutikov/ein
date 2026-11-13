# S1h.4.3 — The trace as the pupil's page: three forms of a view, each on real traces

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **Read-only on the engine:** each form is written out by a script
over a real trace; no source under `ein.rs/`, no grammar file and nothing outside `plans/` and `build/` moves.
Its question, [Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page), is put to the user at
[S1h.4.5](s1h.4.5_the_page_and_the_stop.md).
**Depends on:** [S1h.4.1](s1h.4.1_the_two_parts_and_a_well_formed_formula.md), which puts every step of a run
on one side of the line; [S1h.2.1](../p1h.2_slicing/s1h.2.1_what_a_slice_is_today.md)'s script, for the
third form (if that stage has not run, a filter of the trace by the module of a step's rule is written here,
and the page says so).
**Asked once with:** [P1g.10](../../m1g_logic/p1g.10_school_algebra/README.md#the-questions-for-the-user)'s
question f.

## What it rests on

Question 36 of the calculator's `questions.md`, *the trace as the pupil's page*, and the user's reply to it,
which makes the subject a phase (*"Yes, this is a phase in M1h"*).

The milestone README: *E-17 … `--trace` cannot show an importing theory's steps without the imported
theory's. … Nothing folds a call into one line, "12 ÷ 3 = 4, remainder 0 (116 steps of `std.arith`)". A script
on the prototype filters the trace. P1g.10 met the same thing on an equation (3 steps of the pupil's in
123).* And the phase's first question: *The trace as the pupil's page: a view of a run at one theory's level,
the imported theory's steps folded to a line each. Whether that is a `--trace` option, a property of an
import, or P1h.2's slice drawn another way; an engine change where it is one, with the reference pages moving
with it.*

## What it starts from

- **Today's trace.** `ein solve examples/word_problems/boxes12.ein -G --trace T.md` writes 145 steps
  (2026-11-13); twelve are by rules the three word modules declare. `-l` keeps 137. `-R` clusters a
  section's steps by the entity they are about, and `-F` appends each state's whole KB
  ([`configuration.md`](../../../docs/kernel/configuration.md), the trace's four modifiers).
- **The investigation's filter,** `build/s1f.6.20/pupil.py`: it keeps the steps whose rule is declared in
  the prototype's files, by reading those files, and prints each one's `:why` and first conclusion. Scratch,
  not tracked; its output for the two-step problem is on
  [`word_problems.md` § 2.2](../../m1f_hypothesis_and_documentation/p1f.6_calculator/word_problems.md#22-the-two-step-problem-in-full).
- **A call.** A word rule asks the calculator by concluding a request, `(div 12 3)`, and reads its answer,
  `(quot-of 12 3 4 z)`. Between the two are the calculator's steps. The calculator's own tagged calls
  (`add-walk`, `mul-call`, `div-call`, `cmp-walk`) are named as an interface by
  [S1f.6.30](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md)'s
  first task.

## What it produces

A script under `build/s1h.4.3/` that writes each of the three forms from a real trace or event stream, the
pages it writes for a set of problems, and the third section of `formula_and_calculation.md`.

**The problems.** `boxes12` (two steps, a division with nothing left); the 13-apple problem (a division with
a remainder, where the fold must still show it); `pencils` (three operations, 298 steps on the prototype);
`contra` (a refutation that passes through arithmetic); and P1g.10's linear equation if
[S1g.10.1](../../m1g_logic/p1g.10_school_algebra/s1g.10.1_survey_and_design.md) has banked it.

## Tasks

1. **T1h.4.3.1 — the target, by hand.** For each problem, the page a pupil would hand in, written by hand
   before any script runs: the steps in words, each with its arithmetic on one line. It is the thing each
   form is compared with, and the page keeps it.
2. **T1h.4.3.2 — what a fold needs, found.** Three things the engine has to know for any of the forms, each
   read off the code and a run, and each a row of `findings.md` if it is missing:
   - **which module a rule belongs to,** in the loaded program and not only in the source files (the
     investigation's filter read the files);
   - **where a call starts and ends:** the request fact and the result fact that answers it, and how the
     steps between are told from steps of another call running at the same time;
   - **what one line says of a folded call:** the request and its result rendered through their `:why`
     texts, and a count of the steps folded.
3. **T1h.4.3.3 — form one: an option of `--trace`.** The trace with the steps of every imported module
   folded, a line a call, written by the script for each problem. Which modules fold: every import, or the
   ones the option names. What `-l`, `-R` and `-G` do together with it.
4. **T1h.4.3.4 — form two: a property of an import.** The importing program says which of its imports are
   *below* it, and the trace folds those. The stage writes the programs as they would read and marks each as
   **not parseable today**: it is a keyword on `(import …)`, a grammar change, shown here on real programs
   and proposed to nobody before the stop. What it gives that an option does not: the choice travels with
   the program, so `ein test` and a dump see the same page.
5. **T1h.4.3.5 — form three: the slice drawn another way.** The cut of the answer by S1h.2.1's script,
   printed in the trace's order with the used rules of each module grouped: the problem's module in full,
   each imported module as the calls made into it. How it differs from forms one and two on the same
   problems (a slice leaves out what the answer did not use; a fold leaves out nothing, it only shortens).
6. **T1h.4.3.6 — the three against the target.** For each problem: each form's page beside the hand-written
   one, the lines that match, the lines the form has and the pupil does not (the plan's steps, *the question
   asks*), and the lines the pupil has and no form gives. With the numbers as they print today,
   `(n (n z 1) 2)` for 12, and one sentence on what that costs a reader; whether a number may print as a
   number is [M1g's S1g.6.3](../../m1g_logic/p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md)'s
   question and is not taken here.
7. **T1h.4.3.7 — the page,** with Q-M1h.16 and what each form would change: a row of
   [`configuration.md`](../../../docs/kernel/configuration.md) and the trace's printer; `ein.l`, `ein.y`
   and the grammar's pages; or nothing beyond P1h.2's surfaces.

## Acceptance

- Each problem has its hand-written page and each form's page, written by the script from a run of the day.
- The 13-apple problem's folded division shows its remainder in every form.
- The three things of T2 are each on the page as *the engine has it* or *missing*, with where it was read.
- Form two's programs are marked not parseable, and nothing in `ein.l` or `ein.y` moved.
- No file under `ein.rs/`, `docs/kernel/`, `examples/`, `stdlib/` or `tests/` moved.

## What it must not do

- **Not change `--trace`, the grammar or an import's meaning.**
- **Not keep the filter script as the answer.** The investigation called it *a finding, not a proposal to
  keep the script*; here it is the means of showing three proposals.
- **Not settle which level a view is at.** A problem file has no rule of its own: it imports the word
  theory, which imports the calculator. Whether the page is the word theory's level, with the calculator
  folded, or is chosen by the reader, is shown both ways in T3 and T4 and is part of Q-M1h.16.
- **Not put the question to the user twice.** If P1g.10's S1g.10.4 has ruled question f, this stage follows
  that ruling, writes out only the ruled form, and the stop does not ask again.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to this phase's `findings.md`; questions to
[`open_questions.md`](../open_questions.md) under the next free id. Scratch stays under `build/s1h.4.3/`. The
link check is run before the commit, which changes pages only.
