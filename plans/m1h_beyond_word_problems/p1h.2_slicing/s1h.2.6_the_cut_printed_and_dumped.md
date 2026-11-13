# S1h.2.6 — The cut printed and dumped, the page that says what *used* means, and the close: an engine stage

**Phase:** [P1h.2](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. **An engine stage, on the surfaces.** It is past
the phase's stop: **its tasks are written at [S1h.2.4](s1h.2.4_what_used_means_and_the_stop.md)'s re-plan,**
to the user's rulings, and nothing below designs a row, a file or a view.
**Depends on:** [S1h.2.5](s1h.2.5_the_cut_as_an_object.md); the ruling on
[Q-M1h.13](../open_questions.md#q-m1h13--the-slices-surfaces) (which surfaces, under which option).

## What it rests on

The user, 2026-11-12: *"Ein must allow showing (print, dump) those parts."* The milestone's acceptance: *for a
query's answer, Ein prints and dumps the part of the program it used and the part it did not, and the page
says what "used" means, with the two opening questions answered on programs.*

## What it builds

The surfaces the user chose at the stop, of the three the milestone README names: a read-out section, a
`--dump-answer` file a slice, a view of `ein render program` with the two parts told apart. Each prints
S1h.2.5's cut and computes nothing of its own.

## What it carries, whatever is ruled

[The phase's discipline](README.md#the-discipline), and of it here:

- **The rows shown first.** The stage's first task re-takes S1h.2.4's written-out surfaces on the day's
  binary, beside what the printer will print, before the printer moves.
- **The pages in the same commit as the printer.** A read-out row:
  [`read_out.md`](../../../docs/kernel/inference/read_out.md), `read_out_examples.md` by
  `utils/read_out_examples.py`, and `ein-cli/tests/read_out_reference.rs`. A dump file:
  [`answer_dump.md`](../../../docs/kernel/inference/answer_dump.md), and `answer_round_trip.rs` if a slice
  file is a loadable program. A view:
  [`04_dot_rendering.md`](../../../docs/kernel/ir/03-ein-lang/04_dot_rendering.md), `dot_wellformed.rs`, and
  `utils/ci_container.sh` green on the commit, since a DOT writer changed. A new option: a row of
  [`configuration.md`](../../../docs/kernel/configuration.md) and `config_reference.rs`.
- **The goldens counted and named before blessing.** A surface asked for by an option moves no golden of a
  run that does not pass it; the record says which digests of `corpus_shapes.md5` moved and why, by a
  command.
- **No answer moves,** under every lever (`lever_invariance`) and with the option and without.
- **One tool, consistent answers:** the cut is the same from `ein solve`, from a dump and from the view, and
  under every mode that reaches the state.

## The page, and the close

- **A kernel page that says what *used* means:** the definition ruled, what the cut names where a derivation
  has nothing to point at, the two opening questions each with its table, and what the surfaces print. It is
  a current page of the kernel tree where that tree then is
  ([P1f.20](../../m1f_hypothesis_and_documentation/p1f.20_docs_refactor/README.md) rebuilds `docs/kernel/`),
  held by whatever test holds its neighbours.
- **The findings closed:** every row of this phase's `findings.md` with a ruling, a fix or an owner.
- **Handed on by name:** to [P1h.3](../p1h.3_gaps_assumptions_and_readings/README.md), the surface an answer
  reports its assumptions through; to [P1h.4](../p1h.4_the_formula_and_the_calculation/README.md), the cut
  by module, which is one of the three forms of the pupil's page.
- The phase's README and the milestone's phase table restated to what was built; `AGENTS.md` only where a
  rule, a location or a command changed.

## Acceptance

Written with the tasks. At the least: the milestone's line above, on the surfaces ruled; the sweep shows no
answer moved; each page's test holds; `./run_tests.sh`, then `./run_tests.sh --nightly`, green on each code
commit.

## What it must not do

- **Not add a surface the user did not choose.**
- **Not charge an unasked run:** stdout, stderr and the exit code of a run without the option are
  byte-identical before and after.
- **Not print the complement in full by default** where S1h.2.4 showed it drowns the read-out.
