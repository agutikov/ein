# S1h.3.7 — What the rulings build

**Phase:** [P1h.3](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's stop: **its tasks are
written at [S1h.3.6](s1h.3.6_the_page_and_the_stop.md)'s re-plan,** to the user's rulings, and nothing below
designs a form or a module.
**Depends on:** the rulings on
[Q-M1h.14](../open_questions.md#q-m1h14--which-of-the-banked-forms-change) and
[Q-M1h.15](../open_questions.md#q-m1h15--universal-assumptions-as-a-module).

## What it rests on

The user, 2026-11-12: the banked forms are *"not only and not final"*; *what is not listed is zero* *"could be
a reusable universal assumption"*; and the word theory lives in *"examples for now"* (reply 34).

## What it ships

Only what the user ruled at the stop. Of the things that could be ruled, and what each carries:

- **A banked form changes** (the gap, the assumption, the model's boundary). The changed module and
  problems under `examples/word_problems/`, each shown with its computed answers before it is banked; each
  changed program's corpus entry and claim; the goldens expected to move (`corpus_exits.txt`,
  `corpus_shapes.md5`, the programs' own) counted and named before blessing;
  `examples/word_problems/README.md` restated, its line *"not only and not final"* replaced by what was
  ruled and when. A program in which a hypothesis leads to a request to the calculator is run alone against
  `region.rs`'s monotonicity test before a gate (E-33; `AGENTS.md` § Edits that go together).
- **Universal assumptions become a module.** If beside the examples: the module, the problems that adopt
  it, a corpus entry each. If in the stdlib: everything a stdlib module carries
  ([`AGENTS.md` § Edits that go together](../../../AGENTS.md)): a program for each rule or tight family
  under `tests/stdlib/` with its `:expect`, a refuting rule in two files, `stdlib_coverage.rs`, the mutation
  census with its survivors banked and read, a `:why` on every relation and rule, the manifest, a corpus
  entry a file, and a section of `stdlib/README.md`. A module that reads `absent` says in its header which
  relations may adopt it and why, with the Zebra case, and its tests include the refusal.
- **Nothing changes.** The stage is its close alone.

## The close, whatever is ruled

- `gaps_assumptions_readings.md` gets a closing section: what was ruled, what was built, what was left.
- Every row of this phase's `findings.md` has a ruling, a fix or an owner.
- The list for [F21](../../followups/f21_non_monotone_programs_decided.md) and the witness for
  [M1g's P1g.7](../../m1g_logic/p1g.7_unsatisfiable_against_owing/README.md) are where those two will find
  them; nothing is edited in either but a pointer line.
- The phase's README and the milestone's phase table are restated to what was built.

## Acceptance

Written with the tasks. At the least: every changed or new program holds its claim under `ein test`; no
answer of a program the ruling did not name moved; where a tracked file outside `plans/` changed,
`./run_tests.sh`, then `./run_tests.sh --nightly`, green on the commit.

## What it must not do

- **Not change a form the user did not rule on.**
- **Not move the word theory into the stdlib** (reply 34), whatever is ruled of the assumptions.
- **Not bank a program whose static-check verdict the stop did not show.**
