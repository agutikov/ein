# S1f.7.7 — A written or concluded `(absent k)`: a fact no premise can read

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 2; S1f.5.24's T2 record notes the conclusion half), filed here under
[Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes). Not started.
**Depends on:** nothing. It may edit `ein.y`'s `FactHead` and `ConclusionHead`, so it runs after
[S1f.7.6](s1f.7.6_a_false_fact_with_an_argument.md), not beside it.

## The finding

Recorded on the release binary at `f62e34e1`:

> **`(absent k)` written as a fact, or concluded, stores a fact no premise can read**: it loads, a fact of a
> relation `absent` no premise can read (`PremiseHead` takes no `absent`, which is negation as failure), as
> `:assert (absent (p ?x))` stores one.

So the program can state a fact that no rule can ever match. It is stored, printed and counted, so it can make two
states differ. S1f.5.23 called it the same case as `open`'s.

## The direction the rulings point to

S1f.5.23 (Q-M1f.66 (a)) refused `open` wherever it named a relation that no premise can read. The direction that
agrees is to refuse `absent` as a fact's, a nested fact's and a conclusion's head: `FactHead`, `ArgHead` and
`ConclusionHead` lose it, with rows in `00_ebnf.md`'s table. `absent` stays the negation-as-failure connective in a
premise.

The other direction would let a premise read a relation named `absent`. That clashes with the connective, so it is
a question for the user.

## Tasks

- **T1f.7.7.1, confirmation and the census.** Re-take the finding as a fact, a nested fact, a stored negation and a
  rule's, an hrule's and a macro's conclusion. Each is run on every command, with the states it can split.
- **T1f.7.7.2, the rows.** One witness per way the engine answers. Each is filed `open` and held to today's engine.
- **T1f.7.7.3, the refusal.** In the grammar, with the rows flipped and a test on every command.
- **T1f.7.7.4, the pages.** `01_grammar.md`, `02_patterns.md`, `00_ebnf.md` re-blessed, `06_reserved_names.md`,
  `defined_behaviour.md`, and the stage's records.

## Acceptance

- `absent` is written only as the connective of a premise, and every other place is refused on every command with
  one line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.
