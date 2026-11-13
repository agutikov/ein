# S1h.1.6 — The temporal module, in the stdlib

**Phase:** [P1h.1](README.md) of [M1h](../README.md)
**Status:** planned 2026-11-13, **in outline.** Not started. It is past the phase's stop: **its tasks are
written at [S1h.1.5](s1h.1.5_the_design_page_and_the_stop.md)'s re-plan,** to the user's rulings, and nothing
below designs the module.
**Depends on:** the rulings on
[Q-M1h.5](../open_questions.md#q-m1h5--where-the-temporal-and-causal-modules-live-and-their-names) (where it
lives and its name) and
[Q-M1h.8](../open_questions.md#q-m1h8--the-temporal-theorys-first-cut-and-what-soft-and-unknown-are) (its
first cut); the stdlib's layout as
[S1f.6.31](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.31_the_stdlib_layout.md) leaves it,
and [S1f.6.30](../../m1f_hypothesis_and_documentation/p1f.6_calculator/s1f.6.30_name_clashes_between_imported_theories.md)'s
ruling on name clashes if there is one.

## What it rests on

The user, 2026-11-12: *"Both are separate theories, can be reused and deserve separate files in stdlib"*. The
milestone's acceptance: *two stdlib modules, a temporal and a causal theory, each with what a stdlib module
has (a program per rule under `tests/stdlib/`, an `:expect` in each, the mutation census, a `:why` on every
relation and rule), and each used by a program that is not a word problem.*

## What it ships

The temporal theory as ruled: the relations S1h.1.5's page proposed and the user accepted, written from
[S1h.1.3](s1h.1.3_the_temporal_theory_on_a_prototype.md)'s prototype.

## What it carries, whatever is ruled

What every stdlib module carries here, by [`AGENTS.md` § Edits that go together](../../../AGENTS.md):

- **a program for each rule or tight family under `tests/stdlib/`,** each with an `:expect`, so that
  `ein-infer/tests/stdlib_coverage.rs` finds every rule activated; a rule that refutes gets two files, one
  where it fires and one where it is loaded, activated and satisfied ([`tests/README.md`](../../../tests/README.md));
- **the mutation census:** `utils/stdlib_mutants.py --check`, the survivors banked as a set in
  `tests/mutation_survivors.txt` and each one read;
- **a `:why` on every relation and rule,** checked for how it reads through
  [`09_natural_language.md`](../../../docs/kernel/ir/03-ein-lang/09_natural_language.md)'s ladder;
- **the manifest,** `utils/stdlib_manifest.py --write`;
- **a corpus entry for every new `.ein`,** in `corpus/corpus.toml`, with the runs it is exercised under;
- **a closing rule** that closes the module's relations and names its rules, as the calculator's modules and
  the word theory have, so that a program that imports it adds no candidate to H;
- **a section in [`stdlib/README.md`](../../../stdlib/README.md),** and the `stdlib/` paragraph of `AGENTS.md`
  if a location or a rule changes.

And of the phase: monotone by `ein solve --monotone` on every program that imports it, or the flagged site is
the one the user accepted; every answer of the banked word problems unchanged (they do not import it).

## Acceptance

Written with the tasks. At the least: the module loads from the checkout, from `$EIN_STDLIB` and from the
embedded copy; `stdlib_coverage.rs` is green with its rules counted; the mutation census is taken and banked;
`./run_tests.sh`, then `./run_tests.sh --nightly`, green on the commit.

## What it must not do

- **Not ship a relation the user did not accept** at the stop, and none that no problem of the set uses.
- **Not edit `examples/word_problems/`.**
- **Not depart from the layout ruling.** If S1f.6.31 has not run, the stage says where the file goes under
  today's flat layout and that the move takes it with the rest.
