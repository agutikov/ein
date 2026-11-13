# S1h.4.6 — The pupil's page, built as ruled: an engine stage where it is one

**Phase:** [P1h.4](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. **An engine stage, on the trace,** unless the
ruling makes it none. It is past the phase's stop: **its tasks are written at
[S1h.4.5](s1h.4.5_the_page_and_the_stop.md)'s re-plan,** to the user's ruling, and nothing below designs the
view.
**Depends on:** the ruling on [Q-M1h.16](../open_questions.md#q-m1h16--the-pupils-page) (or P1g.10's ruling
on its question f, if that came first); for the third form,
[P1h.2](../p1h.2_slicing/README.md)'s [S1h.2.5](../p1h.2_slicing/s1h.2.5_the_cut_as_an_object.md).

## What it rests on

The milestone README: *The trace as the pupil's page: a view of a run at one theory's level, the imported
theory's steps folded to a line each. Whether that is a `--trace` option, a property of an import, or
P1h.2's slice drawn another way; an engine change where it is one, with the reference pages moving with it.*

## What it builds, by the ruling

- **An option of `--trace`:** the fold in the trace's printer (`ein-render`'s `trace`), a line a call, with
  what [S1h.4.3](s1h.4.3_the_trace_as_the_pupils_page.md)'s second task found the engine must know: a
  rule's module, and where a call starts and ends.
- **A property of an import:** a grammar change. `ein.l` and `ein.y`; the grammar's pages re-generated
  (`EIN_BLESS=1`), a row in [`01_grammar.md`](../../../docs/kernel/ir/03-ein-lang/01_grammar.md) for the new
  keyword and one in its table for each shape newly refused; what the keyword means to every command that
  reads a program, since every command reads one by one path; and then the fold as above.
- **The slice drawn another way:** no trace change; the page is a rendering of P1h.2's cut, and this stage
  is then a task of [S1h.2.6](../p1h.2_slicing/s1h.2.6_the_cut_printed_and_dumped.md) or follows it.
- **The filter is enough** (the calculator's question 36, option (c)): nothing is built; the script moves to
  `utils/` with its line in `utils/README.md`, and E-17 is closed as ruled.

## What it carries, whatever is ruled

[The phase's discipline](README.md#the-discipline), and of it here:

- **The pages shown first.** The stage's first task re-takes S1h.4.3's written-out pages on the day's
  binary beside what the printer will write, before the printer moves.
- **The reference pages in the same commit:** a row of
  [`configuration.md`](../../../docs/kernel/configuration.md) for a new option, held by
  `config_reference.rs`; the trace's golden (`ein-render/tests/golden_trace.rs`) and the `region-trace`
  digests of `corpus_shapes.md5`, which a run without the option must leave unmoved.
- **No answer moves;** stdout, stderr and the exit code of every run without the option, or without the
  keyword, are byte-identical before and after, by a sweep.
- **One tool, consistent answers:** the folded page says nothing the unfolded trace does not, and a test
  holds the fold to the trace it folds (every folded call's request and result are steps of the whole
  trace, and its count is theirs).
- **E-17** of the calculator's
  [`findings.md`](../../m1f_hypothesis_and_documentation/p1f.6_calculator/findings.md) and **F-2** of
  [P1g.10's](../../m1g_logic/p1g.10_school_algebra/findings.md) are marked with what was built, by a pointer
  line in each.

## Acceptance

Written with the tasks. At the least, where a view is built: the two-step problem's page is the word
theory's steps with each call into the calculator one line, held by a test; the sweep shows no answer moved;
`./run_tests.sh`, then `./run_tests.sh --nightly`, green on each code commit.

## What it must not do

- **Not build a form the user did not rule.**
- **Not change what the unfolded trace prints.**
- **Not print a number as a number** as a side effect. That is another question and another stage's
  ([M1g's S1g.6.3](../../m1g_logic/p1g.6_engine_requirements/s1g.6.3_seeing_nested_facts.md)).
