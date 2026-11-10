# S1f.7.12 — A macro call written as a fact

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-11, from the [analysis of 2026-11-11](README.md#the-analysis-of-2026-11-11), on the user's
ruling of that day (*"yes - it become S1f.7.10 onward"*). Not started.
**Depends on:** [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), which lists every place a macro call is read and
not expanded.

## The finding

Taken on the release binary at `4bf6da73`, 2026-11-11: with `(macro pp (?a) (p ?a))`, a top-level `(pp j)` loads,
exit 0. It is stored as a fact of a relation `pp`, and `(p j)` is not stored: it appears only as a candidate. The
author who wrote the call has neither the fact the macro spells nor an error.

## The direction

A macro's body is a pattern, read in a rule's `:match`. A fact is not a pattern.

- **(a) Refused at load** (the default): a fact, at any depth, or a conclusion whose head is a macro's name, the
  line naming the macro and saying a macro is called in a `:match` or a `:goal`.
- **(b) Expanded**: the call replaced by its body where the body is one ground form. This gives a macro a second
  meaning, and a body with a connective has none as a fact.

## Tasks

- **T1f.7.12.1, confirmation and the census.** Re-take it. Every site a macro's name can head a form that is not
  expanded: a fact, a nested fact, a rule's and an hrule's `:assert`, a claim's model, a `(solve …)` list, an
  import's `:symbols` name used as a relation. The stdlib's macros (`unknown`, `forall`) written as a fact.
- **T1f.7.12.2, the rows**, `open`, held to today's engine.
- **T1f.7.12.3, the fix**, the rows flipped, a test on every command.
- **T1f.7.12.4, the pages**: `01_grammar.md`'s text on macros, `defined_behaviour.md`, the records.

## Acceptance

- No command stores a fact whose relation is a macro's name.
- Every witness is a register row. No corpus answer moves but the witnesses'.
- `./run_tests.sh` green.
