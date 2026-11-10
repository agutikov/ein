# S1f.7.6 — A written `(false k)`: `false` with an argument

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from [S1f.5.23's T2 record](../p1f.5_the_no_owner_items/s1f.5.23_open_is_reserved.md#t2-record-2026-11-10)
(its out-of-scope finding 1), filed here under [Q-M1f.85](../open_questions.md#q-m1f85--which-findings-p1f7-takes).
Not started.
**Depends on:** nothing. It may edit `ein.y`'s `FactHead`, so it does not run beside
[S1f.7.7](s1f.7.7_an_absent_fact.md).

## The finding

Recorded on the release binary at `f62e34e1`:

> **`(false k)` written as a fact kills root**: root dead, R empty, the argument read as nothing. S1f.5.24 refuses
> `(false ?x)` a macro's expansion concludes (*false takes no argument, not 1*) and the grammar a rule's written
> `(false k)`, so a written fact is the one place `false` with an argument is taken.

So `false` with an argument is refused in two places and taken in a third. Where it is taken, the argument is
dropped in silence and the program is a contradiction.

## The direction the rulings point to

The grammar and S1f.5.24 (W68, [Q-M1f.81](../open_questions.md#q-m1f81--what-a-malformed-primitive-is)) refuse
`false` with an argument wherever a rule writes it. The direction that agrees is to refuse it as a written fact too:
`FactHead` takes `false` only at no argument, with a row in `00_ebnf.md`'s table. The census also reads `false` as a
nested fact's head and in a claim.

The other direction would make `(false k)` a fact of an ordinary relation `false`. That gives the word two meanings
by arity, so it is a question for the user.

## Tasks

- **T1f.7.6.1, confirmation and the census.** Re-take the finding. Read `false` with arguments in every place a fact
  or a value is written (a top-level fact, a nested fact, a stored negation, a claim, a `--diff-models` file), each on
  every command.
- **T1f.7.6.2, the rows.** One witness per way the engine answers. Each is filed `open` and held to today's engine.
- **T1f.7.6.3, the refusal.** In the grammar where it can, with the rows flipped and a test on every command.
- **T1f.7.6.4, the pages.** `01_grammar.md`, `00_ebnf.md` re-blessed, `06_reserved_names.md`, `defined_behaviour.md`,
  and the stage's records.

## Acceptance

- `false` with an argument is refused in every place, on every command, with one line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.
