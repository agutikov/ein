# S1f.7.3 — A connective as a premise pattern's argument

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 14. Not started.
**Depends on:** nothing. It extends [S1f.5.24](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md)'s
walk, `ein_ir::from_ir::refuse_malformed_primitives`.

## The finding

Recorded in [S1f.5.24's T2 record](../p1f.5_the_no_owner_items/s1f.5.24_a_macro_body_checked_where_a_rule_is.md#t2-record-2026-11-10)
on the release binary at `dbfd7862`:

> **A connective written as a premise pattern's argument reads nothing, in silence.** `(rule r () :match (and (p
> ?x) (q (and (p ?x)))) …)`: the grammar's `Arg` takes a connective, so a macro can be called there, and the
> loader and the compiler read `(and (p ?x))` as a pattern inside a pattern whose relation is `and`, which no fact
> holds, so `r` never fires (written, and through a macro's argument, `(macro m (?a) (q ?a))` called with a
> conjunction). The same shape is a parse error as a conclusion's or a guard's value, and S1f.5.24 refuses it
> there; in a premise pattern it is the grammar's own, so not this stage's.

So the grammar accepts a shape that has no meaning, and the program loads with a rule that can never fire.

## The direction the rulings point to

S1f.5.24 refused the same shape after macro expansion as a conclusion's value and as a guard's value (W67, W68,
[Q-M1f.81](../open_questions.md#q-m1f81--what-a-malformed-primitive-is)). The direction that agrees is to
refuse it as a premise pattern's argument too, after expansion, with the same line. That covers `and`, `or`, `not`,
`absent` and a guard as an argument, each one T1 shows means nothing there.

The grammar keeps `Arg` taking a connective only if a macro call needs it. T1 reads why `Arg` takes one. If no
macro call needs it, the grammar refuses the written shape at the parse, and the loader refuses only what an
expansion produces.

## Tasks

### T1f.7.3.1 — confirmation, and the class

Re-take the finding. List every connective and primitive that `Arg` admits in a premise pattern, written and
through a macro's body or argument, in a rule, an hrule, an obligation and a goal, each on every command. Find out
why `ein.y`'s `Arg` takes a connective.

### T1f.7.3.2 — the rows

One witness per way the engine answers. Each is filed `open` and held to today's engine.

### T1f.7.3.3 — the refusal

The grammar, the walk, or both, as T1 decides, then the rows flipped. A row is added to `00_ebnf.md`'s table if the
grammar takes the refusal. A test holds every witness on every command.

### T1f.7.3.4 — the pages

[`02_patterns.md`](../../../docs/kernel/ir/03-ein-lang/02_patterns.md), `00_ebnf.md` (re-blessed if the grammar
moves), `defined_behaviour.md` § 4.16, and the stage's records.

## Acceptance

- A connective or primitive with no meaning as a premise pattern's argument is refused on every command, written or
  produced by a macro, with S1f.5.24's line.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.3 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.
