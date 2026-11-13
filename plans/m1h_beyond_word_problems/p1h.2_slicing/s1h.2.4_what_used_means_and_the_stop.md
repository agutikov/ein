# S1h.2.4 — What *used* means, the surfaces shown on real programs, and the stop

**Phase:** [P1h.2](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **It is the phase's stop:** no engine source and nothing a
surface prints changes before the user has read its page and ruled.
**Depends on:** [S1h.2.1](s1h.2.1_what_a_slice_is_today.md) (the script and the cut on the set),
[S1h.2.2](s1h.2.2_assumptions_as_premises.md) and
[S1h.2.3](s1h.2.3_a_restriction_that_was_checked_and_held.md) (the two opening questions).

## What it rests on

The user, 2026-11-12: *"Ein must allow showing (print, dump) those parts."* The user's rule for a change to
what a surface prints (2026-10-16): real programs, with their real computed answers, shown before it is
implemented.

The milestone README's further questions: *whether a slice is of one answer or of a query's every class;
whether "required" means "used by this derivation" or "used by every derivation" (a fact with two
justifications has two slices; a core is an explanation and may vary with rule order, the user's ruling of
2026-11-08, and a slice inherits that unless it is defined as an intersection or a union); what the unused
part of an imported theory is (most of the calculator, for any one sum); and the surface: a read-out section,
a `--dump-answer` file a slice, a view of `ein render program` with the two parts told apart.*

## What it produces

`slicing.md` in this directory, whole: the three earlier sections, the definitions compared, every proposed
surface written out for real programs with content the script computed, and the questions. The commit at the
stop is pages only.

## Tasks

1. **T1h.2.4.1 — *this derivation* against *every derivation*.** Programs of the set in which a fact on the
   answer's path has more than one recorded justification (`alt` events; the config field
   `:record-alternative-justifications` decides whether they are kept). For each, three cuts by the script:
   **(a)** the first justification of each fact; **(b)** the union over every recorded one; **(c)** the
   intersection. How far they differ, in facts and rules, on each program; and for (a), whether it moves
   when two rules of the program change places, which is what *a core is an explanation* predicts. If no
   program of the set has such a fact, one is written, and the page says the corpus had none.
2. **T1h.2.4.2 — one answer against a query's every class.** On `examples/zebra2-minus-15.ein` or another
   program whose query has several classes: the cut of each class, what they share, and what a single cut
   *of the query* would be (their union, with each part marked by the classes that used it). The same for a
   state reached by several commitments.
3. **T1h.2.4.3 — what the cut names where a derivation has nothing to point at.** The three cases the
   earlier stages left: a premise that is an `absent` (S1h.2.1, on `gaps.ein`): the relation read and the
   pattern that found nothing; a restriction that was tested and held (S1h.2.3): in the used part under a
   name of its own, or apart; a fact written with a `:rule` and no premises (F24's witness): a leaf. Each
   proposed on its program.
4. **T1h.2.4.4 — the unused part, shown.** For one sum, the calculator's unused rules are most of the
   module. Three ways to show a complement, each written out for `bob1.ein` and `unused.ein`: every item
   listed; a count a module with the problem's own unused facts listed; the problem's own alone, the
   imported modules summed in a line. With the lines each takes against stdout's 100.
5. **T1h.2.4.5 — the surfaces, each on real programs.** Written out in full for `unused.ein`, `boxes12.ein`
   and one zebra program, as each would print, the content from the script:
   - **a read-out section:** rows under a query's answer, or a form asked for by an option, as `-H` and
     `--models` are; what is cut at 100 lines and what the last line says;
   - **`--dump-answer`:** *a file a slice*. Where the files sit in the directory
     ([`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md)), what a file holds, and one property
     the page tests on every banked problem: **the used part alone, loaded as a program, answers the query
     as the whole did.** Where it does not (a closing rule the answer never fired, an `absent` that held
     because of a fact now gone), the page shows the program and says what the file then is;
   - **`ein render program`:** the view with the two parts told apart, as DOT rendered for one program.
   For each: the option that would ask for it, counted against the CLI's options as
   [`configuration.md`](../../../docs/kernel/configuration.md) lists them, and what it costs a run that asks
   and one that does not (nothing).
6. **T1h.2.4.6 — the questions,** each beside its programs:
   [Q-M1h.11](../open_questions.md#q-m1h11--what-used-means), what *used* means;
   [Q-M1h.12](../open_questions.md#q-m1h12--is-a-restriction-that-was-tested-and-held-recorded), whether a
   tested restriction is recorded;
   [Q-M1h.13](../open_questions.md#q-m1h13--the-slices-surfaces), which surfaces. And the findings so far,
   each with its witness.
7. **T1h.2.4.7 — the stop.** The user reads the page. The rulings are recorded on it in the user's words and
   in [`open_questions.md`](../open_questions.md).
8. **T1h.2.4.8 — the re-plan.** [S1h.2.5](s1h.2.5_the_cut_as_an_object.md) and
   [S1h.2.6](s1h.2.6_the_cut_printed_and_dumped.md) are restated to the rulings and given their tasks.

## Acceptance

- The three cuts of T1 are on the page for at least one program where they differ, each computed.
- Every proposed surface is on the page written out for real programs, with nothing typed that the script
  did not compute.
- The property *the used part alone answers the same* is tested on every banked problem, and each failure is
  on the page with its reason.
- The user has ruled, or has said which questions wait; the page and `open_questions.md` record it in the
  user's words.
- The two later stage files have tasks, written to the rulings.
- No file under `ein.rs/`, `docs/kernel/`, `examples/`, `stdlib/` or `tests/` moved.

## What it must not do

- **Not change a printer, a dump or a view.**
- **Not take a default** on a question the user has not answered.
- **Not call the first justification's cut *required*.** It is *used by this derivation* until Q-M1h.11 is
  ruled.
- **Not propose a surface that makes an unasked run pay.**

## The standing rules

Questions go to [`open_questions.md`](../open_questions.md) under the next free id; findings to this phase's
`findings.md`. A ruling is recorded as the user gave it and may be reopened by a later finding. Scratch stays
under `build/s1h.2.4/`. The link check is run before the commit, which changes pages only.
