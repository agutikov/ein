# S1f.7.1 — A goal's nested pattern whose relation is a variable: refused, as in a rule

**Phase:** [P1f.7](README.md) (The wrong answers P1f.5's stages found outside their scope)
**Status:** new 2026-11-10, from S1f.5.28's item 12. Not started.
**Depends on:** [S1f.7.5](s1f.7.5_a_macro_called_in_a_goal.md), so the census reads a goal after its macros are
expanded.

## The finding

Recorded in [S1f.5.21's T1 record](../p1f.5_the_no_owner_items/s1f.5.21_a_compile_error_in_eins_words.md#t1-record-2026-11-10)
on the release binary at `864b16ed`, not re-checked since:

> A query's goal that writes a pattern inside a pattern whose relation is a variable, `(query :goal (p (?R ?x)))`,
> loads, and the compiler lowers the inner pattern as an opaque slot that matches nothing, so `(p (g k))` gives the
> goal no row, in silence (`goal-free-head-nested.ein`, … *the goal has no row: undetermined here*). In a rule the
> same shape is refused at load since M1e2 S1e2.2.24 (W24's `nested-unbound-head`).

So one shape gets two answers: refused in a rule, and in a goal answered as if no fact matched. That breaks
*one tool, consistent answers*, and the goal's answer is wrong, since `(p (g k))` is the fact the goal asks about.

## The direction the rulings point to

The user ruled on Q-M1e2.16 (1), and again on Q-M1e2.18, that this shape is refused at load in a rule
(S1e2.2.24, W24). The direction that agrees is the same refusal in a goal: one line in S1f.5.21's spelling, on
every command, naming the query and the clause.

The other direction would give the shape a meaning in both places: a nested pattern whose relation the match binds
from the fact. That reverses the W24 ruling, so it is a question for the user, not a default.

## Tasks

### T1f.7.1.1 — confirmation, and every check a goal skips

Re-take the finding. Then list every load-time check a rule's `:match` gets (`refuse_no_fact_holds`,
`refuse_nested_disjunctions`, `refuse_malformed_primitives`, the variable-width limit, `validate_open`, the
reserved names and any others the loader holds) and whether a goal's premise gets it, each with a witness. A
skipped check that changes an answer is part of this stage if it is the same shape, and becomes a new P1f.7 stage
if it is not.

### T1f.7.1.2 — the rows

One witness per way the engine answers, on every command that reads a goal (every `solve` question, `test` with a
claim, `oracle`, the views, `kb save` and a saved container). Each is filed `open` and held to today's engine.

### T1f.7.1.3 — the refusal

The goal is held to the rule's checks at load. The rows are flipped to `fixed`. A test holds every witness on every
command.

### T1f.7.1.4 — the pages

[`01_grammar.md` § Query](../../../docs/kernel/ir/03-ein-lang/01_grammar.md#query), `defined_behaviour.md` § 4,
and the stage's records.

## Acceptance

- A goal's nested pattern whose relation is a variable is refused on every command with the rule's line.
- Every check a rule's `:match` gets and a goal's premise skipped is listed, and each one that moves an answer is
  fixed here or is a P1f.7 stage.
- Every witness is a register row, filed `open` before the fix and `fixed` after it. No corpus answer moves.
- `./run_tests.sh` green.

## The analysis of 2026-11-11

The finding was re-taken on the binaries at `4bf6da73` before the stage started, and reproduced. What that found
beyond this file is the S1f.7.1 row of [the phase's table](README.md#the-analysis-of-2026-11-11), and T1 takes it in.
