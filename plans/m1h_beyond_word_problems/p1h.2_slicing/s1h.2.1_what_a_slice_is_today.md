# S1h.2.1 — What a slice is today: the trace's, the core's, and the cut of a program computed by a script

**Phase:** [P1h.2](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13. Not started. **Read-only on the engine:** it runs the binary and writes a
script and a page; no source under `ein.rs/` and nothing outside `plans/`, `build/` and its own script moves.
**Depends on:** the word problems as banked under
[`examples/word_problems/`](../../../examples/word_problems/README.md).

## What it rests on

The user, 2026-11-12: *"question and inference of the answer cuts the program (which is a model of a problem)
into 2 parts used/unused, required/superfluous. Ein must allow showing (print, dump) those parts."*

The milestone README, on where the phase starts: *What exists to start from: every derived fact records its
justification, so the facts and rules an answer rests on can be collected by walking it; `--trace -l` prints
"the goal-relevant slice" of a trace; a dead node's core is a slice of another kind, the facts a contradiction
rests on; the banked `unused.ein` and `bystander.ein` are problems with a fact no question needs. What does
not exist: the cut as an object, of the **program** (its facts, its rules, its imported modules' rules) and
not of the trace, printed and dumped; and its complement, the part the answer did not touch.*

## What it produces

- **`slice.py`,** a script under `build/s1h.2.1/` that computes the cut of a program by one answer from what
  a run already writes, with no change to the engine. It is the phase's prototype and, afterwards, the
  independent check the engine's own cut is held to
  ([S1h.2.5](s1h.2.5_the_cut_as_an_object.md)); if it is kept past the phase it moves to `utils/` at that
  stage, with its line in `utils/README.md`.
- **The first section of `slicing.md`** in this directory: what each existing slice is, by definition and on
  programs, and the cut the script computes, for each program of the set below.

**The programs.** From `examples/word_problems/`: `bob1`, `unused`, `bystander`, `boxes12`, `pencils`,
`contra`, `gap`, `gap_assume`. From outside the word problems, so that the cut is not shaped by one theory:
`examples/zebra2.ein` (one state, reached by a commitment), `examples/zebra2-minus-15.ein` (a query with
several classes), one program of `tests/stdlib/` from each of two refutation pairs, and
[F24](../../followups/f24_written_provenance.md)'s witness of a fact written with a `:rule`.

## Tasks

1. **T1h.2.1.1 — `--trace -l`, by definition and by count.** What the pruning keeps, read off
   `ein-render/src/trace/relevance.rs`: the seeds (the facts of the query's relations and of the `:hrules`
   targets), the walk back along provenance, the redundant re-derivations dropped, the firings marked
   conditional. Then the table of the phase's [README](README.md#what-it-starts-from) re-taken on every
   program of the set (`ein solve FILE -G [-l] --trace T.md`, steps counted), with two things said of it: it
   is a slice of the **firings** and names no fact or rule that did not fire; and on a dead root it keeps the
   path to the query's relations and not the contradiction (`contra.ein`: 3 of 23 steps on 2026-11-13).
2. **T1h.2.1.2 — a core, by definition and on programs.** `contra.ein`'s `unsat core` and `source` rows and
   `refuted by`; a dead commitment's entry in `--dump-answer`'s `nogoods.json` on a zebra program. What a
   core is a slice of (the **given facts** a refutation rests on, and the rule of its last step), what it
   leaves out (the rules between), and the two rulings it lives under: it is taken only where a surface
   prints one, and it may vary with rule order.
3. **T1h.2.1.3 — the script.** Input: a program, and for one run of it the `--events` stream
   (`fire`: `rule`, `premises`, `derived`; `alt`: a further justification) and the program as loaded
   (`ein saturate FILE --dump`, or a `--dump-answer` state file, which writes the program with its imports
   inlined). For one answer, the facts that match a query's goal in one state, it walks back through the
   justifications and writes four lists: **the given facts used**, each with its `:source`; **the rules
   used**, each with the module that declares it; and the complement of each. Under a commitment, the
   commitment's own facts are a third kind and are listed apart. The first justification of a fact is the
   one walked, and the script says so in its output; [S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md)
   takes the others.
4. **T1h.2.1.4 — the cut, on the set.** For each program: the four lists, or their counts where a list is
   long. What the page must show among them:
   - `unused.ein`: Alice's 7, Carol's 9 and the box are in the unused facts, with their sentences;
   - `bob1.ein`: of the calculator's rules, how many one subtraction used
     (`grep -c '^(rule ' stdlib/arith.ein` is the whole, re-counted on the day);
   - `boxes12.ein`: `(model-counts-only)` is in the used facts, and in `bob1.ein`'s it is not;
   - `gap.ein`: an answer that rests on an `absent`. `(not-stated …)` was derived because a fact was *not*
     there, and a walk through premises finds no fact for that premise. The page says what the script wrote
     for it and leaves what it should write to S1h.2.4;
   - a zebra state: which of the puzzle's clues the answer to its query used, and which commitment;
   - F24's witness: a fact written with a `:rule` has no premises, so the walk stops at it.
5. **T1h.2.1.5 — the three slices side by side.** One table over the set: what `--trace -l` kept, what a
   core held where there is one, what the script's cut holds; and for each pair, what one has that the other
   does not.
6. **T1h.2.1.6 — the page,** every number with its command and date.

## Acceptance

- The script runs on every program of the set and writes the four lists; its output for each is on the page
  or beside it under `build/s1h.2.1/`.
- The definitions of the trace's slice and of a core are on the page with the source file or kernel page
  each is read from.
- The six observations of T4 are each on the page with the program and the lines that show it.
- No file under `ein.rs/`, `examples/`, `stdlib/` or `tests/` moved.

## What it must not do

- **Not change the engine,** an event's schema or any surface.
- **Not decide what *used* means.** It walks the first justification because that is what a run records
  first, and says so; the choice is
  [Q-M1h.11](../open_questions.md#q-m1h11--what-used-means).
- **Not treat the script's output as the answer to either opening question.** Those are
  [S1h.2.2](s1h.2.2_assumptions_as_premises.md)'s and
  [S1h.2.3](s1h.2.3_a_restriction_that_was_checked_and_held.md)'s.

## What it may find

- **That `--events` does not carry enough** to rebuild a justification under a commitment, where a node
  resumes root's saturation and narrates only its own part
  ([`events.md`](../../../docs/kernel/inference/events.md) on narration). Then the script reads the
  `--dump-answer` state file's provenance as well, and the page says which run artefact holds what.
- **That a rule's module is not recoverable from a run,** only from the source files. Then the script reads
  the sources for it, and the gap is a row of `findings.md`: S1h.2.5 and
  [S1h.4.3](../p1h.4_the_formula_and_the_calculation/s1h.4.3_the_trace_as_the_pupils_page.md) both need it.

## The standing rules

A wrong answer of the engine met on the way is a row in `corpus/wrong_answers.toml`, filed `open` with its
witness before anything fixes it. Findings go to `findings.md` in this directory, opened at the first;
questions to [`open_questions.md`](../open_questions.md) under the next free id. Scratch stays under
`build/s1h.2.1/`. The link check is run before the commit, which changes pages only.
